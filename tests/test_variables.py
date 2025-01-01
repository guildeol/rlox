from rlox import rlox

def test_should_define_uninitialized_variable():
    source = 'var a;\n' \
             'print a;'
    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['nil'])

def test_should_define_initialized_variable():
    source = 'var a = 12;\n' \
             'print a;'
    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['12'])

def test_should_reassign_variable():
    source = 'var a = 12;\n'            \
             'a = "Another thing";\n'   \
             'print a;'
    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['"Another thing"'])

def test_should_shadow_variable():
    source = 'var a = 52;           \n' \
             '{                     \n' \
             '  var a = "Shadow";   \n' \
             '  print a;            \n' \
             '}                     \n' \
             'print a;'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['"Shadow"', '52' ])

def test_should_get_enclosed_variable():
    source = 'var a = "outer";      \n' \
             '{                     \n' \
             '  print a;            \n' \
             '}                     \n' \
             'print a;'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['"outer"', '"outer"' ])

def test_should_update_outer_variable():
    source = 'var a = 0;            \n' \
             '{                     \n' \
             '  a = a + 1;          \n' \
             '}                     \n' \
             'print a;'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['1'])
