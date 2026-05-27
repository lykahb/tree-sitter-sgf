import unittest

import tree_sitter_sgf
from tree_sitter import Language, Parser


class BindingTest(unittest.TestCase):
    def test_can_load_and_parse_sgf(self):
        language = Language(tree_sitter_sgf.language())
        parser = Parser(language)

        tree = parser.parse(b"(;GM[1]FF[4]SZ[19];B[pd];W[dd])")
        self.assertFalse(tree.root_node.has_error)


if __name__ == "__main__":
    unittest.main()
