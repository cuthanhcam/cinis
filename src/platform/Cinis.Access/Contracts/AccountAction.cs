namespace Cinis.Access.Contracts;

/// <summary>Access actions only; financial validity remains Rust-owned.</summary>
public enum AccountAction { Read, Trade, Administer }
