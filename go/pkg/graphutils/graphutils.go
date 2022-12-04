package graphutils

type Graphable[T any] interface {
	Iterate() []T
	Neighbors(T) ([]T, error)
}

type WeightedPath[T any] struct {
	from          *Node[T]
	to            *Node[T]
	weight        uint64
	bidirectional bool
}

type Node[T any] struct {
	value     T
	neighbors []WeightedPath[T]
}

type Graph[T any] struct {
	nodes []Node[T]
}
