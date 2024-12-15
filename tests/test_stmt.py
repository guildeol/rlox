import pytest
from tests.rlox import rlox

def assert_success(result, stdout, expected_stdout):
    """Helper function to check successful execution and expected output."""
    assert result == rlox.SUCCESS, 'Statement evaluation finished with unexpected result'
    assert stdout.strip() == expected_stdout

def assert_failure(result, stderr):
    assert result != rlox.SUCCESS
    assert stderr != '', "Code failed without error message!"

def test_should_print_binary_expr():
    result, stdout, _ = rlox.run('print 1 + 2;')
    assert_success(result, stdout, '3')

def test_should_print_grouping_expr():
    result, stdout, _ = rlox.run('print 5 * (1 + 2);')
    assert_success(result, stdout, '15')

def test_should_print_unary_expr():
    result, stdout, _ = rlox.run('print -5789;')
    assert_success(result, stdout, '-5789')

def test_should_print_uninitialized_variable():
    source = 'var a;\nprint a;'
    result, stdout, _ = rlox.run(source)
    assert_success(result, stdout, 'nil')

def test_should_print_initialized_variable():
    source = 'var a = 12;\nprint a;'
    result, stdout, _ = rlox.run(source)
    assert_success(result, stdout, '12')

def test_should_reassign_variable():
    source = 'var a = 12;\na = "Another thing";\nprint a;'
    result, stdout, _ = rlox.run(source)
    assert_success(result, stdout, 'Another thing')

def test_should_handle_syntax_error():
    result, _, stderr = rlox.run('print unterminated_statement')
    assert_failure(result, stderr)
