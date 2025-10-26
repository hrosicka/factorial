# 🔢 Safe Factorial Calculator

A simple, interactive console program written in **Rust** that calculates the factorial of a given non-negative integer. The program features robust input handling and crucial overflow checking to ensure safe mathematical operations.

## ✨ Features

* **Interactive Loop:** Allows the user to perform multiple calculations without restarting the program.
* **Safe Overflow Prevention:** Automatically checks if the input number exceeds the safe limit for a `u64` type factorial (**20**), preventing silent data corruption.
* **Robust Input Handling:** Gracefully handles non-numeric or invalid inputs.
* **Clean Exit:** The program can be cleanly terminated by typing `exit`.
* **Iterative Calculation:** Uses an iterative approach with `checked_mul()` for safer arithmetic operations.

---

## 📝 License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details. Free to use, modify, and distribute as needed.


