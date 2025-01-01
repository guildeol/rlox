import pytest

from tests.rlox import rlox

def test_should_execute_while_loop():
    source = 'var i = 0;            \n' \
             'while (i < 3)         \n' \
             '{                     \n' \
             '    print i;          \n' \
             '    i = i + 1;        \n' \
             '}'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['0', '1', '2'])

def test_should_not_execute_while_loop():
    source = 'while (false)         \n' \
             '{                     \n' \
             '    print "Error";          \n' \
             '}'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, [])
