// Tool descriptions
pub const RUN_COMMAND_DESCRIPTION: &str = "A system command execution tool that allows running shell commands with full system access. 

SECRET HANDLING: 
- Output containing secrets will be redacted and shown as placeholders like [REDACTED_SECRET:rule-id:hash]
- You can use these placeholders in subsequent commands - they will be automatically restored to actual values before execution
- Example: If you see 'export API_KEY=[REDACTED_SECRET:api-key:abc123]', you can use '[REDACTED_SECRET:api-key:abc123]' in later commands

If the command's output exceeds 300 lines the result will be truncated and the full output will be saved to a file in the current directory";

pub const VIEW_DESCRIPTION: &str = "View the contents of a file or list the contents of a directory. Can read entire files or specific line ranges.

SECRET HANDLING:
- File contents containing secrets will be redacted and shown as placeholders like [REDACTED_SECRET:rule-id:hash]
- These placeholders represent actual secret values that are safely stored for later use
- You can reference these placeholders when working with the file content

A maximum of 300 lines will be shown at a time, the rest will be truncated.";

pub const STR_REPLACE_DESCRIPTION: &str = "Replace a specific string in a file with new text. The old_str must match exactly including whitespace and indentation.

SECRET HANDLING:
- You can use secret placeholders like [REDACTED_SECRET:rule-id:hash] in both old_str and new_str parameters
- These placeholders will be automatically restored to actual secret values before performing the replacement
- This allows you to safely work with secret values without exposing them

When replacing code, ensure the new text maintains proper syntax, indentation, and follows the codebase style.";

pub const CREATE_DESCRIPTION: &str = "Create a new file with the specified content. Will fail if file already exists. When creating code, ensure the new text has proper syntax, indentation, and follows the codebase style. Parent directories will be created automatically if they don't exist.";

pub const GENERATE_CODE_DESCRIPTION: &str = "Advanced Generate/Edit devops configurations and infrastructure as code with suggested file names using a given prompt. This code generation/editing only works for Terraform, Kubernetes, Dockerfile, and Github Actions. If save_files is true, the generated files will be saved to the filesystem. The printed shell output will redact any secrets, will be replaced with a placeholder [REDACTED_SECRET:rule-id:short-hash]

IMPORTANT: When breaking down large projects into multiple generation steps, always include previously generated files in the 'context' parameter to maintain coherent references and consistent structure across all generated files.";

pub const REMOTE_CODE_SEARCH_DESCRIPTION: &str = "Query remote configurations and infrastructure as code indexed in Stakpak using natural language. This function uses a smart retrival system to find relevant code blocks with a relevance score, not just keyword matching. This function is useful for finding code blocks that are not in your local filesystem.";

pub const LOCAL_CODE_SEARCH_DESCRIPTION: &str = r#"Search for local code blocks using multiple keywords.
IMPORTANT: this tool ONLY search through local Terraform, Kubernetes, Dockerfile, and Github Actions code.
This tool searches through the locally indexed code blocks using text matching against names, types, content, and file paths. Blocks matching multiple keywords are ranked higher in the results. It can also show dependencies and dependents of matching blocks. If no index is found, it will build one first."#;

pub const SEARCH_DOCS_DESCRIPTION: &str = "Web search for technical documentation. This includes documentation for cloud-native tools, cloud providers, development frameworks, release notes, and other technical resources.";

pub const SEARCH_MEMORY_DESCRIPTION: &str = "Search your memory for relevant information from previous conversations and code generation steps to accelerate request fulfillment.";

pub const READ_RULEBOOK_DESCRIPTION: &str = "Read and retrieve the contents of a rulebook using its URI. This tool allows you to access and read rulebooks that contain play books, guidelines, policies, or rules defined by the user.";

pub const GENERATE_PASSWORD_DESCRIPTION: &str = "Generate a secure password with the specified constraints. The password will be generated using the following constraints:
- Length of the password (default: 15)
- No symbols (default: false)
";

// Parameter descriptions
pub const COMMAND_PARAM_DESCRIPTION: &str = "The shell command to execute";
pub const WORK_DIR_PARAM_DESCRIPTION: &str = "Optional working directory for command execution";

pub const PATH_PARAM_DESCRIPTION: &str = "The path to the file or directory to view";
pub const VIEW_RANGE_PARAM_DESCRIPTION: &str = "Optional line range to view [start_line, end_line]. Line numbers are 1-indexed. Use -1 for end_line to read to end of file.";

pub const FILE_PATH_PARAM_DESCRIPTION: &str = "The path to the file to modify";
pub const OLD_STR_PARAM_DESCRIPTION: &str =
    "The exact text to replace (must match exactly, including whitespace and indentation)";
pub const NEW_STR_PARAM_DESCRIPTION: &str = "The new text to insert in place of the old text. When replacing code, ensure the new text maintains proper syntax, indentation, and follows the codebase style.";
pub const REPLACE_ALL_PARAM_DESCRIPTION: &str =
    "Whether to replace all occurrences of the old text in the file (default: false)";

pub const CREATE_PATH_PARAM_DESCRIPTION: &str = "The path where the new file should be created";
pub const FILE_TEXT_PARAM_DESCRIPTION: &str = "The content to write to the new file, when creating code, ensure the new text has proper syntax, indentation, and follows the codebase style.";

pub const GENERATE_PROMPT_PARAM_DESCRIPTION: &str = "Prompt to use to generate code, this should be as detailed as possible. Make sure to specify the paths of the files to be created or modified if you want to save changes to the filesystem.";
pub const PROVISIONER_PARAM_DESCRIPTION: &str =
    "Type of code to generate one of Dockerfile, Kubernetes, Terraform, GithubActions";
pub const SAVE_FILES_PARAM_DESCRIPTION: &str =
    "Whether to save the generated files to the filesystem (default: false)";
pub const CONTEXT_PARAM_DESCRIPTION: &str = "Optional list of file paths to include as context for the generation. CRITICAL: When generating code in multiple steps (breaking down large projects), always include previously generated files from earlier steps to ensure consistent references, imports, and overall project coherence. Add any files you want to edit, or that you want to use as context for the generation (default: empty)";

pub const REMOTE_CODE_SEARCH_QUERY_PARAM_DESCRIPTION: &str = "The natural language query to find relevant code blocks, the more detailed the query the better the results will be";
pub const REMOTE_CODE_SEARCH_LIMIT_PARAM_DESCRIPTION: &str =
    "The maximum number of results to return (default: 10)";

// pub const SEARCH_DOCS_QUERY_PARAM_DESCRIPTION: &str = "The natural language query to find relevant technical documentation on the internet, the more detailed the query the better the results will be";
pub const SEARCH_DOCS_LIMIT_PARAM_DESCRIPTION: &str =
    "The maximum number of results to return (default: 5, max: 5)";
pub const SEARCH_DOCS_KEYWORDS_PARAM_DESCRIPTION: &str = "List of keywords to search for in the documentation. Searches against the url, title, description, and content of documentation chunks.";
pub const SEARCH_DOCS_EXCLUDE_KEYWORDS_PARAM_DESCRIPTION: &str = "List of keywords to exclude from the search results. This is useful for filtering out documentation sources that are not relevant to the query.";

pub const LOCAL_CODE_SEARCH_KEYWORDS_PARAM_DESCRIPTION: &str = "List of keywords to search for in code blocks. Searches against block names, types, content, and file paths. Blocks matching multiple keywords will be ranked higher than those matching only one keyword.";
pub const LOCAL_CODE_SEARCH_LIMIT_PARAM_DESCRIPTION: &str =
    "Maximum number of results to return (default: 10)";
pub const LOCAL_CODE_SEARCH_SHOW_DEPENDENCIES_PARAM_DESCRIPTION: &str =
    "Whether to show dependencies and dependents for each matching block (default: false)";

pub const SEARCH_MEMORY_LIMIT_PARAM_DESCRIPTION: &str = "The maximum number of results to return";
pub const SEARCH_MEMORY_KEYWORDS_PARAM_DESCRIPTION: &str = "List of keywords to search for in your memory. Searches against the title, tags, and content of your memory.";

pub const READ_RULEBOOK_URI_PARAM_DESCRIPTION: &str =
    "The URI of the rulebook to read. This should be a valid URI pointing to a rulebook document.";

pub const LENGTH_PARAM_DESCRIPTION: &str = "The length of the password to generate";

pub const NO_SYMBOLS_PARAM_DESCRIPTION: &str =
    "Whether to disallow symbols in the password (default: false)";
