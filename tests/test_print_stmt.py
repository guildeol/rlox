import pytest

from tests.rlox import rlox

def test_should_print_binary_expr():
    result, stdout, _ = rlox.run('print 1 + 2;')
    assert rlox.succeeded(result, stdout, ['3'])

def test_should_print_grouping_expr():
    result, stdout, _ = rlox.run('print 5 * (1 + 2);')
    assert rlox.succeeded(result, stdout, ['15'])

def test_should_print_unary_expr():
    result, stdout, _ = rlox.run('print -5789;')
    assert rlox.succeeded(result, stdout, ['-5789'])

def test_should_handle_syntax_error():
    result, _, stderr = rlox.run('print unterminated_statement')
    assert rlox.failed(result, stderr)
