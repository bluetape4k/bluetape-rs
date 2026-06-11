package compressionmatrix

import (
	"bytes"
	"fmt"
	"math/rand/v2"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/bluetape4k/bluetape-go/compression"
)

type payload struct {
	kind string
	size string
	data []byte
}

type compressorCase struct {
	name string
	c    compression.Compressor
}

func sameConditionPayloads() []payload {
	const fixtureDir = "/tmp/bluetape-compression-bench/payloads"
	var payloads []payload
	for _, kind := range []string{"json", "text", "binary", "random"} {
		for _, size := range []string{"small", "medium", "large"} {
			data, err := os.ReadFile(filepath.Join(fixtureDir, kind+"-"+size+".bin"))
			if err != nil {
				panic(err)
			}
			payloads = append(payloads, payload{kind: kind, size: size, data: data})
		}
	}
	return payloads
}

func generatedPayloads() []payload {
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

func sameConditionCompressors() []compressorCase {
	var cases []compressorCase
	for _, c := range compression.All() {
		cases = append(cases, compressorCase{name: c.Name(), c: c})
	}
	return cases
}

func BenchmarkSameConditionCompressors(b *testing.B) {
	for _, cc := range sameConditionCompressors() {
		for _, p := range sameConditionPayloads() {
			name := fmt.Sprintf("%s/%s/%s", cc.name, p.kind, p.size)
			b.Run("compress/"+name, func(b *testing.B) {
				compressed, err := cc.c.Compress(p.data)
				if err != nil {
					b.Fatal(err)
				}
				b.ReportAllocs()
				b.SetBytes(int64(len(p.data)))
				b.ResetTimer()
				for range b.N {
					if _, err := cc.c.Compress(p.data); err != nil {
						b.Fatal(err)
					}
				}
				b.ReportMetric(float64(len(compressed)), "compressed_bytes")
				b.ReportMetric(float64(len(compressed))/float64(len(p.data)), "ratio")
			})
			b.Run("decompress/"+name, func(b *testing.B) {
				compressed, err := cc.c.Compress(p.data)
				if err != nil {
					b.Fatal(err)
				}
				restored, err := cc.c.Decompress(compressed)
				if err != nil {
					b.Fatal(err)
				}
				if !bytes.Equal(restored, p.data) {
					b.Fatalf("round trip mismatch: %s", name)
				}
				b.ReportAllocs()
				b.SetBytes(int64(len(p.data)))
				b.ResetTimer()
				for range b.N {
					if _, err := cc.c.Decompress(compressed); err != nil {
						b.Fatal(err)
					}
				}
				b.ReportMetric(float64(len(compressed)), "compressed_bytes")
				b.ReportMetric(float64(len(compressed))/float64(len(p.data)), "ratio")
			})
		}
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
