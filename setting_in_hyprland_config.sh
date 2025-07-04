#!/bin/bash

# Script to inject hypremoji rules into Hyprland configuration
# Usage: ./setting_in_hyprland_config.sh <config_file>

CONFIG_FILE="$1"

if [[ ! -f "$CONFIG_FILE" ]]; then
    echo "Error: Configuration file not found: $CONFIG_FILE"
    exit 1
fi

# Function to check if a similar rule already exists
check_existing_rule() {
    local command_keys=("$@")
    local search_term
    
    # Determine the search term based on the type of command
    if [[ " ${command_keys[*]} " =~ " bind " ]]; then
        search_term="hypremoji"  # For bind commands
    elif [[ " ${command_keys[*]} " =~ " windowrule" ]]; then
        search_term="HyprEmoji"  # For windowrules (window title)
    else
        search_term="hypremoji"  # Fallback
    fi
    
    for key in "${command_keys[@]}"; do
        # Look for uncommented lines containing the command + search term
        if grep -q "^[[:space:]]*$key.*$search_term" "$CONFIG_FILE" && \
           ! grep -q "^[[:space:]]*#.*$key.*$search_term" "$CONFIG_FILE"; then
            return 0  # Active rule already exists
        fi
    done
    return 1  # Rule does not exist
}

# Improved function to inject commands
inject_commands_improved() {
    local command_keys=("$@")
    local commands_to_inject=()
    local temp_file=$(mktemp)
    local found_section=false
    local injected=false
    
    # Define commands to inject based on type
    if [[ " ${command_keys[*]} " =~ " bind " ]]; then
        commands_to_inject=(
            ""
            "# SUPER + PERIOD to open Hypremoji"
            "bind = \$mainMod, period, exec, hypremoji"
        )
    elif [[ " ${command_keys[*]} " =~ " windowrule" ]]; then
        commands_to_inject=(
            ""
            "# WindowRules for HyprEmojis"
            "windowrulev2 = float, title:^(HyprEmoji)\$"
        )
    fi
    
    local in_target_section=false
    
    while IFS= read -r line; do
        # Check if the current line is a target command
        local is_target_command=false
        for key in "${command_keys[@]}"; do
            if [[ "$line" =~ ^[[:space:]]*$key ]]; then
                is_target_command=true
                found_section=true
                in_target_section=true
                break
            fi
        done
        
        # If inside the target section
        if [[ "$in_target_section" == true ]] && [[ "$is_target_command" == false ]]; then
            # If the line is empty - inject here
            if [[ -z "${line// }" ]]; then
                if [[ "$injected" == false ]]; then
                    for cmd in "${commands_to_inject[@]}"; do
                        echo "$cmd" >> "$temp_file"
                    done
                    injected=true
                fi
                in_target_section=false
            elif [[ "$line" =~ ^[[:space:]]*# ]]; then
                # It's a comment - check if it's a commented command
                local is_commented_command=false
                for key in "${command_keys[@]}"; do
                    if [[ "$line" =~ ^[[:space:]]*#.*$key ]]; then
                        is_commented_command=true
                        break
                    fi
                done
                if [[ "$is_commented_command" == false ]]; then
                    if [[ "$injected" == false ]]; then
                        for cmd in "${commands_to_inject[@]}"; do
                            echo "$cmd" >> "$temp_file"
                        done
                        echo "" >> "$temp_file"  # Extra newline for spacing
                        injected=true
                    fi
                    in_target_section=false
                else
                    # It's a commented command, stay in section but don't inject here
                    :
                fi
            else
                # Not a target command, comment or empty line = end of section
                if [[ "$injected" == false ]]; then
                    # Inject before this line
                    for cmd in "${commands_to_inject[@]}"; do
                        echo "$cmd" >> "$temp_file"
                    done
                    injected=true
                fi
                in_target_section=false
            fi
        fi
        
        echo "$line" >> "$temp_file"
    done < "$CONFIG_FILE"
    
    # If we ended inside the section and didn't inject yet, inject at the end
    if [[ "$in_target_section" == true ]] && [[ "$injected" == false ]]; then
        for cmd in "${commands_to_inject[@]}"; do
            echo "$cmd" >> "$temp_file"
        done
        injected=true
    fi
    
    # If no section found, append at the end
    if [[ "$found_section" == false ]]; then
        echo "" >> "$temp_file"
        for cmd in "${commands_to_inject[@]}"; do
            echo "$cmd" >> "$temp_file"
        done
    fi
    
    mv "$temp_file" "$CONFIG_FILE"
}

# Main function
main() {
    echo "Processing Hyprland configuration: $CONFIG_FILE"

    # Verificar si hay algo que modificar
    needs_bind_injection=false
    needs_windowrule_injection=false

    if ! check_existing_rule "bind" "bindm" "bindel" "bindl"; then
        needs_bind_injection=true
    fi

    if ! check_existing_rule "windowrule" "windowrulev2"; then
        needs_windowrule_injection=true
    fi

    if [[ "$needs_bind_injection" == false && "$needs_windowrule_injection" == false ]]; then
        echo "Nothing to change. Configuration already up to date. No backup created."
        return 0
    fi

    # Crear respaldo solo si se va a modificar
    BACKUP_PATH="$(dirname "$CONFIG_FILE")/hyprland-backup-$(date +%Y%m%d-%H%M%S).conf"
    cp "$CONFIG_FILE" "$BACKUP_PATH"
    echo "Backup created at: $BACKUP_PATH"

    # Inyectar bind si es necesario
    if [[ "$needs_bind_injection" == true ]]; then
        echo "Adding keybind for hypremoji..."
        inject_commands_improved "bind"
    else
        echo "Keybind for hypremoji already exists, skipping..."
    fi

    # Inyectar windowrules si es necesario
    if [[ "$needs_windowrule_injection" == true ]]; then
        echo "Adding windowrules for hypremoji..."
        inject_commands_improved "windowrule" "windowrulev2"
    else
        echo "Windowrules for hypremoji already exist, skipping..."
    fi

    echo "Hyprland configuration successfully updated!"
}

# Run main function
main
