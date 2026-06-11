package main

import (
	"crypto/sha256"
	"encoding/hex"
	"flag"
	"fmt"
	"math/rand/v2"
	"os"
	"path/filepath"
	"strings"
)

type payload struct {
	kind string
	size string
	data []byte
}

func main() {
	dir := flag.String("output-dir", "/tmp/bluetape-compression-bench/payloads", "directory for generated payload fixtures")
	manifest := flag.String("manifest", "../../../docs/benchmark/compression-fixtures-manifest.csv", "CSV manifest path")
	flag.Parse()

	if err := os.MkdirAll(*dir, 0o755); err != nil {
		panic(err)
	}
	var manifestRows []string
	manifestRows = append(manifestRows, "file,bytes,sha256,first32_hex")
	for _, p := range payloads() {
		file := p.kind + "-" + p.size + ".bin"
		path := filepath.Join(*dir, file)
		if err := os.WriteFile(path, p.data, 0o644); err != nil {
			panic(err)
		}
		sum := sha256.Sum256(p.data)
		first := p.data
		if len(first) > 32 {
			first = first[:32]
		}
		manifestRows = append(manifestRows, fmt.Sprintf("%s,%d,%s,%s", file, len(p.data), hex.EncodeToString(sum[:]), hex.EncodeToString(first)))
		fmt.Printf("%s,%d\n", path, len(p.data))
	}
	if err := os.MkdirAll(filepath.Dir(*manifest), 0o755); err != nil {
		panic(err)
	}
	if err := os.WriteFile(*manifest, []byte(strings.Join(manifestRows, "\n")+"\n"), 0o644); err != nil {
		panic(err)
	}
}

func payloads() []payload {
	return []payload{
		{"json", "small", jsonPayload(1024)},
		{"json", "medium", jsonPayload(64 * 1024)},
		{"json", "large", jsonPayload(512 * 1024)},
		{"text", "small", textPayload(1024)},
		{"text", "medium", textPayload(64 * 1024)},
		{"text", "large", textPayload(512 * 1024)},
		{"binary", "small", binaryPayload(1024)},
		{"binary", "medium", binaryPayload(64 * 1024)},
		{"binary", "large", binaryPayload(512 * 1024)},
		{"random", "small", randomPayload(1024)},
		{"random", "medium", randomPayload(64 * 1024)},
		{"random", "large", randomPayload(512 * 1024)},
	}
}

func jsonPayload(target int) []byte {
	var b strings.Builder
	b.WriteByte('[')
	i := 0
	for b.Len() < target-96 {
		if i > 0 {
			b.WriteByte(',')
		}
		fmt.Fprintf(&b, `{"id":%d,"name":"item-%06d","active":%t,"tags":["blue","tape","bench"],"score":%d}`,
			i, i, i%2 == 0, (i*37)%10000)
		i++
	}
	b.WriteByte(']')
	return padOrTrim([]byte(b.String()), target)
}

func textPayload(target int) []byte {
	line := "bluetape compression benchmark payload with repeated readable text and stable entropy.\n"
	return []byte(strings.Repeat(line, target/len(line)+1)[:target])
}

func binaryPayload(target int) []byte {
	out := make([]byte, target)
	for i := range out {
		out[i] = byte((i*31 + i/7 + 17) % 251)
	}
	return out
}

func randomPayload(target int) []byte {
	r := rand.New(rand.NewPCG(0x5eed, 0x0ddba11))
	out := make([]byte, target)
	for i := range out {
		out[i] = byte(r.Uint64())
	}
	return out
}

func padOrTrim(data []byte, target int) []byte {
	if len(data) >= target {
		return data[:target]
	}
	out := make([]byte, target)
	copy(out, data)
	for i := len(data); i < target; i++ {
		out[i] = ' '
	}
	return out
}
