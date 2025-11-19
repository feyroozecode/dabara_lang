# Changelog - Dabara v0.2.0

## 🎉 Version 0.2.0 - Functions Are Here! (November 19, 2025)

### 🚀 Major Features

#### ✨ Function Execution (CRITICAL)
- **Implemented function calls** - Functions can now be called and executed!
- **Local variable scoping** - Proper scope isolation with scope stack
- **Return statements** - Added `mayar` keyword for returning values
- **Recursive functions** - Full support for recursion (factorial example works!)
- **Parameter binding** - Function parameters properly bound to arguments

### 🐛 Bug Fixes

- **Fixed else branch execution** - Else branches now properly execute their statements
- **Fixed variable scoping** - Variables are now properly isolated in function scopes

### 🧪 Testing

- **Added 9 new unit tests** for function execution
- **All 38 tests passing** (29 existing + 9 new)
- **Created demo script** `test_008_functions_working.ha` showcasing all function features

### 📝 Code Changes

- **lexer.rs**: Added `mayar` (return) keyword
- **parser.rs**: Added Return statement AST and fixed else branch bug  
- **interpreter.rs**: Complete refactor to scope stack + function execution

### 🎯 What You Can Now Do

```hausa
# Recursive factorial!
aiki factorial(n) {
  idan n == 0 {
    mayar 1
  } amma {
    mayar n * factorial(n - 1)
  }
}
rubuta factorial(5)  # Output: 120
```

### 📊 Stats

- **~431 lines** of code added/modified
- **3 critical blockers** resolved:
  1. ✅ Function execution
  2. ✅ Local scoping
  3. ✅ Return statements

---

## Previous Versions

### Version 0.1.3
- Basic conditionals (`idan`/`amma`)
- Variables and arithmetic
- String concatenation
- Lists support

### Version 0.1.2  
- Initial release
- Basic lexer and parser
- Simple statement execution

---

**Dabara is now a real programming language!** 🎊
