# SigmaOS-timer

SigmaOS integration component for **GNOME Timer**.

## Commands

```bash
sigmaos-timer install   # install package + deploy defaults
sigmaos-timer deploy    # copy config templates to $HOME
sigmaos-timer status    # binary/package/config report
sigmaos-timer reset     # remove deployed config
sigmaos-timer open      # launch the application
```

## Design

- Shell-based component manager (Omarchy-style per-app repos)
- Secure defaults; no secrets stored
- Deterministic `test.sh` with isolated `$HOME`
