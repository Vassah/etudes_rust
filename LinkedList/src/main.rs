/// # Data Type Definition
/// Recursive type implementing singly linked list.
enum ListNode<T> {
	Empty,
	Node(T, Box<ListNode>),
}

function isEmpty(node: Empty) -> bool {
	true;
}
function isEmpty(node: Node<T>) -> bool {
	false;
}

function prepend(val: T, node: ListNode<T>)->Node<T> {
	Node(val, node)
}

function remove(node: Empty) -> Empty {
	return Empty;
}