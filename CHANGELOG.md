### What's changed in v1.5.0

* feat: generate-deploy-key command (#53) (by @patrickleet)

  This adds a new subcommand to vnext that simplifies setting up deploy keys for GitHub repositories:
    
    - Automatically detects current repository owner and name when possible
    - Generates an Ed25519 SSH key pair
    - Sets the private key as a GitHub repository secret
    - Adds the public key as a deploy key to the repository
    - Provides improved error handling and user prompts
    - Includes comprehensive documentation in README
    
    This feature is particularly useful for CI/CD workflows where the default GITHUB_TOKEN
    cannot trigger other workflows, such as when using the shared workflow at
    unbounded-tech/workflow-vnext-tag.
