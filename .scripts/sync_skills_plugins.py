#!/usr/bin/env python3
import os
import json
import pathlib

def sync_skills():
    cursor_skills_dir = pathlib.Path('/Users/reneboygarcia/.cursor/skills')
    gemini_legacy_skills_dir = pathlib.Path('/Users/reneboygarcia/.gemini/skills')
    gemini_skills_dir = pathlib.Path('/Users/reneboygarcia/.gemini/config/skills')
    
    gemini_skills_dir.mkdir(parents=True, exist_ok=True)
    
    # 1. Sync from Cursor skills
    if cursor_skills_dir.exists():
        for item in cursor_skills_dir.iterdir():
            if item.name.startswith('.') or item.name == '.DS_Store':
                continue
                
            target = gemini_skills_dir / item.name
            if not target.exists() and not target.is_symlink():
                print(f"Linking skill: {item.name}")
                try:
                    os.symlink(item, target)
                except Exception as e:
                    print(f"Error linking skill {item.name}: {e}")

    # 2. Sync from legacy/custom Gemini skills (e.g. performance-oracle)
    if gemini_legacy_skills_dir.exists():
        for item in gemini_legacy_skills_dir.iterdir():
            if item.name.startswith('.') or item.name == '.DS_Store':
                continue
                
            target = gemini_skills_dir / item.name
            if not target.exists() and not target.is_symlink():
                print(f"Linking custom/legacy Gemini skill: {item.name}")
                try:
                    os.symlink(item, target)
                except Exception as e:
                    print(f"Error linking legacy skill {item.name}: {e}")

def sync_plugins_from_dir(src_dir, gemini_plugins_dir):
    if not src_dir.exists():
        return
        
    for plugin_dir in src_dir.iterdir():
        if not plugin_dir.is_dir() or plugin_dir.name.startswith('.'):
            continue
            
        # Find the single hashed directory inside plugin_dir, or check if plugin_dir contains the plugin files directly
        subdirs = [d for d in plugin_dir.iterdir() if d.is_dir() and not d.name.startswith('.')]
        
        # If there's a hashed subdirectory (like in cursor-public cache)
        if len(subdirs) == 1 and len(subdirs[0].name) > 30:
            hash_dir = subdirs[0]
        else:
            # Otherwise use the plugin_dir itself (e.g. local plugin structure)
            hash_dir = plugin_dir
            
        # Locate plugin.json
        plugin_json_candidates = [
            hash_dir / '.cursor-plugin' / 'plugin.json',
            hash_dir / '.claude-plugin' / 'plugin.json',
            hash_dir / '.plugin' / 'plugin.json',
            hash_dir / '.codex-plugin' / 'plugin.json',
            hash_dir / 'plugin.json'
        ]
        
        plugin_json_path = None
        for candidate in plugin_json_candidates:
            if candidate.exists():
                plugin_json_path = candidate
                break
                
        if not plugin_json_path:
            print(f"Could not find plugin.json for plugin: {plugin_dir.name}")
            continue
            
        target_plugin_dir = gemini_plugins_dir / plugin_dir.name
        target_plugin_dir.mkdir(parents=True, exist_ok=True)
        
        # 1. Symlink plugin.json
        target_plugin_json = target_plugin_dir / 'plugin.json'
        if not target_plugin_json.exists() and not target_plugin_json.is_symlink():
            print(f"Linking plugin.json for {plugin_dir.name}")
            try:
                os.symlink(plugin_json_path, target_plugin_json)
            except Exception as e:
                print(f"Error linking plugin.json for {plugin_dir.name}: {e}")
                
        # 2. Symlink skills, agents, commands directories if they exist
        for subdir_name in ['skills', 'agents', 'commands']:
            source_subdir = hash_dir / subdir_name
            if source_subdir.exists() and source_subdir.is_dir():
                target_subdir = target_plugin_dir / subdir_name
                if not target_subdir.exists() and not target_subdir.is_symlink():
                    print(f"Linking {subdir_name} for plugin {plugin_dir.name}")
                    try:
                        os.symlink(source_subdir, target_subdir)
                    except Exception as e:
                        print(f"Error linking {subdir_name} for plugin {plugin_dir.name}: {e}")
                        
        # 3. Create installed_version.json if missing
        target_ver_json = target_plugin_dir / 'installed_version.json'
        if not target_ver_json.exists():
            try:
                with open(target_ver_json, 'w') as f:
                    json.dump({"version": "1.0.0"}, f)
            except Exception as e:
                print(f"Error creating installed_version.json for {plugin_dir.name}: {e}")

def sync_plugins():
    gemini_plugins_dir = pathlib.Path('/Users/reneboygarcia/.gemini/config/plugins')
    gemini_plugins_dir.mkdir(parents=True, exist_ok=True)
    
    # Sync public plugins (cached)
    sync_plugins_from_dir(pathlib.Path('/Users/reneboygarcia/.cursor/plugins/cache/cursor-public'), gemini_plugins_dir)
    
    # Sync local plugins
    sync_plugins_from_dir(pathlib.Path('/Users/reneboygarcia/.cursor/plugins/local'), gemini_plugins_dir)

def sync_plugin_agents_as_skills():
    gemini_plugins_dir = pathlib.Path('/Users/reneboygarcia/.gemini/config/plugins')
    gemini_skills_dir = pathlib.Path('/Users/reneboygarcia/.gemini/config/skills')
    
    if not gemini_plugins_dir.exists():
        return
        
    gemini_skills_dir.mkdir(parents=True, exist_ok=True)
    
    for plugin_dir in gemini_plugins_dir.iterdir():
        if not plugin_dir.is_dir() or plugin_dir.name.startswith('.'):
            continue
            
        agents_dir = plugin_dir / 'agents'
        if agents_dir.exists() and agents_dir.is_dir():
            print(f"Syncing agents from plugin {plugin_dir.name} as skills...")
            for agent_file in agents_dir.iterdir():
                if agent_file.is_file() and agent_file.suffix == '.md' and not agent_file.name.startswith('.'):
                    skill_name = agent_file.stem
                    target_skill_dir = gemini_skills_dir / skill_name
                    target_skill_dir.mkdir(parents=True, exist_ok=True)
                    
                    target_skill_md = target_skill_dir / 'SKILL.md'
                    if not target_skill_md.exists() and not target_skill_md.is_symlink():
                        print(f"Linking agent as skill: {skill_name}")
                        try:
                            os.symlink(agent_file, target_skill_md)
                        except Exception as e:
                            print(f"Error linking agent {skill_name}: {e}")

if __name__ == '__main__':
    print("Syncing Cursor skills and plugins to Gemini config...")
    sync_skills()
    sync_plugins()
    sync_plugin_agents_as_skills()
    print("Sync completed.")
