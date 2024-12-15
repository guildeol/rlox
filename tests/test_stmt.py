import pytest

from repl import repl

def test_should_print_binary_expr():
    result, stdout, _ = repl.run('print 1 + 2;')
    assert(result == repl.SUCCESS), 'Statement evaluation finished with unexpected result'
    assert(stdout == '3')

def test_should_print_grouping_expr():
    result, stdout, _ = repl.run('print 5 * (1 + 2);')
    assert(result == repl.SUCCESS), 'Statement evaluation finished with unexpected result'
    assert(stdout == '15')

def test_should_print_unary_expr():
    result, stdout, _ = repl.run('print -5789;')
    assert(result == repl.SUCCESS), 'Statement evaluation finished with unexpected result'
    assert(stdout == '-5789')

def test_should_print_uninitialized_variable():
    source = 'var a;' \
             'print a;'

    result, stdout, _ = repl.run(source)
    assert(result == repl.SUCCESS), 'Statement evaluation finished with unexpected result'
    assert(stdout == 'nil')

def test_should_print_initialized_variable():
    source = 'var a = 12;' \
             'print a;'

    result, stdout, _ = repl.run(source)
    assert(result == repl.SUCCESS), 'Statement evaluation finished with unexpected result'
    assert(stdout == '12')

def test_should_reassign_variable():
    source = 'var a = 12;'          \
             'a = "Another thing";' \
             'print a;'

    result, stdout, _ = repl.run(source)
    assert(result == repl.SUCCESS), 'Statement evaluation finished with unexpected result'
    assert(stdout == 'Another thing')
