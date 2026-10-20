namespace Cinis.Access.Contracts;

/// <summary>Access actions only; financial validity remains Rust-owned.</summary>
public enum AccountAction
{
    /// <summary>Read an authorized account view; this does not grant trade permission.</summary>
    Read,
    /// <summary>Submit trading intents for the specific account.</summary>
    Trade,
    /// <summary>Perform an explicitly granted administrative action; no role inheritance is implied.</summary>
    Administer
}
