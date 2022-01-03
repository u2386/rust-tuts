# coding: utf-8

from typing import Optional, Any

class Node:

    def __init__(self, value=None):
        self.prev: Optional[Node] = None
        self.next: Optional[Node] = None
        self.value = value


class DoublyLinkedList:

    def __init__(self):
        self.head = Node()
        self.tail = Node()

        self.head.next = self.tail
        self.tail.prev = self.head


def push(link, node):
    node.prev = link.tail.prev
    node.next = link.tail

    link.tail.prev.next = node
    link.tail.prev = node


def pop(link):
    node = link.head.next
    if node.value is None:
        return None

    link.head.next = node.next
    node.next.prev = link.head

    node.next = None
    node.prev = None
    return node.value


def main():
    link = DoublyLinkedList()
    print(pop(link))

    for i in range(10):
        push(link, Node(i))

    v = pop(link)
    while v is not None:
        print(v)
        v = pop(link)

    print(pop(link))


if __name__ == '__main__':
    main()

