# EmmyLua for Zed

<div align="center">

**Advanced Lua language support for Zed Editor with EmmyLua annotations**

[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

[Features](#-features) • [Installation](#-installation) • [Usage](#-usage) • [Configuration](#-configuration) • [Examples](examples/)

</div>

## ✨ Features

### 🚀 Language Server Integration
- **Intelligent Code Completion**: Context-aware suggestions with EmmyLua annotation support
- **Diagnostics & Linting**: Real-time error detection and warnings
- **Go to Definition**: Navigate to function and variable definitions instantly
- **Find References**: Locate all usages of symbols across your workspace
- **Hover Documentation**: View type information and documentation on hover
- **Signature Help**: Display function signatures as you type
- **Workspace Configuration**: Customizable language server settings

### 🎨 Syntax Highlighting
- Full Lua 5.1-5.5 syntax support
- EmmyLua annotation highlighting (`---@param`, `---@return`, `---@class`, etc.)
- Semantic token support for enhanced visual clarity
- String interpolation and escape sequence highlighting
- Built-in function and keyword highlighting

### 🔧 Code Editing
- **Smart Brackets**: Auto-closing and auto-pairing for `()`, `[]`, `{}`, `[[]]`, and quotes
- **Block Comments**: Support for `--[[  ]]` multi-line comments
- **Code Folding**: Collapse functions, tables, and control structures
- **Indentation**: Intelligent auto-indent based on Lua syntax
- **Word Selection**: Smart selection of Lua identifiers
- **Rich Code Snippets**: 30+ snippets for common Lua patterns and EmmyLua annotations

### 📐 Code Navigation
- **Outline View**: Browse document structure with symbols (functions, variables, classes)
- **Breadcrumbs**: Navigate through nested scopes
- **Text Objects**: Navigate and select Lua-specific code structures (functions, loops, conditionals, tables)
- **Symbol Search**: Quick navigation to any symbol in your workspace

### 🛠️ Development Tools
- **Task Runner**: Pre-configured tasks for running Lua scripts
- **Code Actions**: Quick fixes and refactoring suggestions
- **Multiple Lua Versions**: Support for Lua 5.1, 5.2, 5.3, 5.4, and LuaJIT

## 📦 Installation

### From Zed Extensions

1. Open Zed
2. Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on macOS)
3. Type "zed: extensions"
4. Search for "EmmyLua"
5. Click Install

### Development Mode

```sh
# Clone the repository
git clone https://github.com/EmmyLuaLs/Zed-EmmyLua.git
cd Zed-EmmyLua

# Build the extension
cargo build --release --target wasm32-wasi

# Install to Zed
# Open Zed, press Ctrl+Shift+P, and select "zed: install dev extension"
# Then select this directory
```

## 🚀 Usage

### EmmyLua Annotations

This extension fully supports EmmyLua annotations for enhanced type checking and documentation:

```lua
---@class Person
---@field name string
---@field age number
local Person = {}

---Creates a new person
---@param name string The person's name
---@param age number The person's age
---@return Person The created person instance
function Person.new(name, age)
    local self = setmetatable({}, { __index = Person })
    self.name = name
    self.age = age
    return self
end

---@param person Person
---@return string
function Person.greet(person)
    return "Hello, " .. person.name .. "!"
end
```

## 🎯 Supported Features

- ✅ Syntax highlighting (Tree-sitter)
- ✅ Code completion
- ✅ Diagnostics
- ✅ Hover information
- ✅ Signature help
- ✅ Go to definition
- ✅ Find references
- ✅ Document symbols
- ✅ Workspace symbols
- ✅ Code formatting (via external formatter)
- ✅ Code actions
- ✅ Rename symbol
- ✅ Folding ranges
- ✅ Selection ranges
- ✅ EmmyLua annotations
- ✅ Type inference
- ✅ Semantic tokens

## 🔧 Requirements

- **Zed Editor**: Version 0.100.0 or higher
- **EmmyLua Language Server**: Automatically downloaded and installed

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [EmmyLua Language Server](https://github.com/EmmyLuaLs/emmylua-analyzer-rust) - The powerful language server
- [Tree-sitter Lua](https://github.com/tree-sitter-grammars/tree-sitter-lua) - Syntax parsing
- [Zed](https://zed.dev) - The next-generation code editor

## 📚 Resources

- [EmmyLua Documentation](https://emmylua.github.io/)
- [Lua Documentation](https://www.lua.org/manual/)
- [Zed Extension Guide](https://zed.dev/docs/extensions)

---

<div align="center">
Made with ❤️ by the EmmyLua community
</div>
