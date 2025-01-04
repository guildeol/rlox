import pytest

from tests.rlox import rlox

def test_should_execute_then_branch():
    source = 'if (true)         \n' \
             '  print "OK";     \n' \
             'else              \n' \
             '  print "Error";'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['"OK"'])

def test_should_execute_else_branch():
    source = 'if (false)        \n' \
             '  print "Error";  \n' \
             'else              \n' \
             '  print "OK";'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['"OK"'])

def test_should_execute_lonely_then_branch():
    source = 'if (true)         \n' \
             '  print "OK";'

    result, stdout, _ = rlox.run(source)
    assert rlox.succeeded(result, stdout, ['"OK"'])
