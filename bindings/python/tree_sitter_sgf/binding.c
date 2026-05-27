#include <Python.h>

typedef struct TSLanguage TSLanguage;
const TSLanguage *tree_sitter_sgf(void);

static PyObject *binding_language(PyObject *Py_UNUSED(self), PyObject *Py_UNUSED(args)) {
    return PyCapsule_New((void *)tree_sitter_sgf(), "tree_sitter.Language", NULL);
}

static PyMethodDef methods[] = {
    {"language", binding_language, METH_NOARGS, "Get the tree-sitter language for this grammar."},
    {NULL, NULL, 0, NULL},
};

static struct PyModuleDef module = {
    .m_base = PyModuleDef_HEAD_INIT,
    .m_name = "_binding",
    .m_doc = NULL,
    .m_size = -1,
    .m_methods = methods,
};

PyMODINIT_FUNC PyInit__binding(void) {
    return PyModule_Create(&module);
}
