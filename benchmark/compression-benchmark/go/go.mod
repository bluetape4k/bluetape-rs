module bluetape-compression-bench

go 1.26.3

require github.com/bluetape4k/bluetape-go v0.0.0

require (
	github.com/golang/snappy v1.0.0 // indirect
	github.com/klauspost/compress v1.18.6 // indirect
	github.com/pierrec/lz4/v4 v4.1.27 // indirect
)

replace github.com/bluetape4k/bluetape-go => ../../../../../../bluetape-go
