*   **Version** : 1.0.0
*   **Date** : 17 September 2025
*   **Author** : Christophe Duy Quang Nguyen
*   **Vibe Coding Engine** : Gemini 2.5 Pro, Google
---
# **An Introduction to `cdqnLang`**

## **1. Welcome to `cdqnLang`**

`cdqnLang` is a modern programming language designed from the ground up for clarity, safety, and power. While it is a capable general-purpose language, it is specifically engineered to build the next generation of smart, immutable systems.

Its philosophy is simple: code should be easy to read, predictable in its behavior, and impossible to misunderstand. To achieve this, `cdqnLang` combines a clean, indentation-based syntax with a strict functional purity model, guided by an AI-powered development environment.

## **2. The Core Principles**

Three core principles define the `cdqnLang` experience:

1.  **Purity and Predictability**: All functions in `cdqnLang` are **pure**. This is the language's most important rule. A pure function is a function that, given the same input, will *always* return the same output and has no side effects (like modifying external data, writing to a file, or printing to the console). This makes your code incredibly easy to test, debug, and reason about.

2.  **Clarity and Explicitness**: The syntax is designed to be unambiguous. All variables have explicit types, and code blocks are clearly defined without relying on a forest of brackets. Intuitive UTF-8 symbols (`←` for assignment, `→` for flow) make the code's intent clear at a glance.

3.  **AI-Assisted Development**: `cdqnLang` is written in the **`ProxyAgent`**, an AI environment that acts as your coding co-pilot. It helps you learn the language, generates code from natural language descriptions, and provides powerful debugging tools that leverage the language's pure and immutable nature.

## **3. The `ProxyAgent`: Your Development Partner**

The `ProxyAgent` is your primary interface for writing `cdqnLang`. It is more than a text editor; it is an active participant in your development process.

*   **Learn Interactively**: Describe what you want to achieve, and the `ProxyAgent` will generate the correct `cdqnLang` code, explaining the logic and syntax.
*   **Go Deeper**: At any time, you can ask the `ProxyAgent` to show you the high-performance **Rust** code that your `cdqnLang` transpiles to, providing a seamless path to understanding systems programming.
*   **"Time-Travel" Debugging**: Because all functions are pure, the `ProxyAgent` can perfectly replay the execution of your program, allowing you to inspect the state of your application at any point in time to find bugs.

## **4. Language Syntax: A Guided Tour**

### **a. Variables and Assignment (`←`)**

All variables must be declared with their type. The leftward arrow (`←`) is the only way to assign a value.

```cdqnlang
// Syntax: type: variable_name ← value

// A single assignment
string: greeting ← "Hello, world."

// Multiple assignments on one line
int: x, y, z ← 10, 20, 30
```

### **b. Code Blocks and Indentation**

`cdqnLang` uses indentation to define code blocks, not brackets (`{}`). A block is opened by a keyword (like `fn` or `if`) and is explicitly closed with a forward slash `/` followed by the same keyword. All code within a block must be indented by **2 spaces**.

```cdqnlang
fn my_block()
  // This line is inside the block.
  // This one is too.
/fn // This closes the block.
```

### **c. Pure Functions**

All functions you write are pure. They are defined with the `fn` keyword. You must specify the types for all parameters and for the return value. The rightward arrow (`→`) indicates the return type.

```cdqnlang
// This pure function takes two integers and returns their sum.
// It will always return the same result for the same inputs.
fn add(int: a, int: b) → int
  int: result ← a + b
  return result
/fn
```

### **d. Handling Side Effects: Describing Actions**

If functions are pure, how do you interact with the outside world?

You don't—at least, your functions don't. Instead of *performing* an action, your functions return a data structure (like a `KDU`) that **describes the action you want to happen**. It is the job of the **`cdqnRuntime`** to execute these descriptions.

This keeps your application logic pure and testable, while isolating the unpredictable parts of the system.

```cdqnlang
// This function does NOT print to the console.
// It returns a KDU that tells the runtime to print the message.
fn create_print_instruction(string: message) → KDU
  KDU: instruction ← KDU.new(
    action: "console.print",
    payload: message
  )
  return instruction
/fn
```

### **e. Conditional Logic (`→`)**

The rightward arrow (`→`) is used to create clean, linear conditional chains, replacing traditional `else if` and `else` structures.

```cdqnlang
int: age ← 30
string: life_stage

if age > 65
  life_stage ← "Senior"
→ (age > 18) // This is the 'else if'
  life_stage ← "Adult"
→ () // This is the final 'else'
  life_stage ← "Minor"
/if

// The variable 'life_stage' now holds the value "Adult".
```

### **f. Loops**

Loops follow the same block structure. A common use case is to transform one list into another without modifying the original.

```cdqnlang
list[int]: original_numbers ← [1, 2, 3]
list[int]: doubled_numbers ← []

for int: num in original_numbers
  doubled_numbers.append(num * 2)
/for

// 'doubled_numbers' is now [2, 4, 6].
// 'original_numbers' remains unchanged.
```

## **5. Next Steps**

This document provides the foundational design of `cdqnLang`. As the `cdqn Kernel (cdqnK)` and `cdqnRuntime` are developed, this documentation will be expanded to include details on their native libraries and how they seamlessly integrate with the language, allowing you to build powerful, verifiable, and intelligent systems.
