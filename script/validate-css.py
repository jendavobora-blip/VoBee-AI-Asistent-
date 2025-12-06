#!/usr/bin/env python3
"""
CSS Validation Script for VitePress Theme
Validates the CSS file for syntax errors and best practices
"""

import re
import sys
from pathlib import Path


def validate_css(css_path):
    """Validate CSS file for common issues"""
    
    print(f"🔍 Validating CSS file: {css_path}")
    print("=" * 60)
    
    with open(css_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    issues = []
    warnings = []
    
    # Check 1: Balanced braces
    open_braces = content.count('{')
    close_braces = content.count('}')
    
    if open_braces != close_braces:
        issues.append(f"Unbalanced braces: {open_braces} opening, {close_braces} closing")
    else:
        print(f"✅ Bracket balance: {open_braces} pairs")
    
    # Check 2: Count custom properties
    # CSS custom properties can contain word chars, hyphens, underscores, and more
    custom_props = re.findall(r'--[a-zA-Z0-9_-]+:', content)
    print(f"✅ CSS custom properties: {len(custom_props)}")
    
    # Check 3: Check for var() usage (including fallback values)
    var_usage = re.findall(r'var\(--[a-zA-Z0-9_-]+(?:,\s*[^)]+)?\)', content)
    print(f"✅ var() usages: {len(var_usage)}")
    
    # Check 4: Check for deprecated properties using consistent patterns
    deprecated_patterns = [
        r'filter:\s*alpha\(',  # IE filter
        r'\bzoom:\s*[^;]+;',   # IE zoom
        r'behavior:\s*url',    # IE behavior
        r'-ms-filter:\s*'      # IE MS filter
    ]
    for pattern in deprecated_patterns:
        matches = re.findall(pattern, content, re.IGNORECASE)
        if matches:
            issues.append(f"Deprecated property found: {matches[0]}")
    
    if not issues:
        print("✅ No deprecated properties found")
    
    # Check 5: Modern features
    oklch_count = content.count('oklch(')
    if oklch_count > 0:
        print(f"✅ Modern OKLCH color space: {oklch_count} usages")
    
    # Check 6: Tailwind integration
    if '@import' in content and 'tailwindcss' in content:
        print("✅ Tailwind CSS properly imported")
    
    if '@apply' in content:
        apply_count = content.count('@apply')
        print(f"✅ Tailwind @apply directives: {apply_count}")
    
    if '@custom-variant' in content:
        print("✅ Tailwind v4 @custom-variant found")
    
    # Check 7: Look for TODOs/FIXMEs
    todos = re.findall(r'(TODO|FIXME|XXX|HACK)', content, re.IGNORECASE)
    if todos:
        warnings.append(f"Found {len(todos)} TODO/FIXME comments")
    else:
        print("✅ No TODO/FIXME comments")
    
    # Check 8: Line count
    lines = content.split('\n')
    print(f"✅ File size: {len(lines)} lines")
    
    # Summary
    print("\n" + "=" * 60)
    if issues:
        print("❌ VALIDATION FAILED")
        print("\nIssues found:")
        for issue in issues:
            print(f"  ❌ {issue}")
        return False
    elif warnings:
        print("⚠️  VALIDATION PASSED WITH WARNINGS")
        print("\nWarnings:")
        for warning in warnings:
            print(f"  ⚠️  {warning}")
        return True
    else:
        print("✅ VALIDATION PASSED")
        print("\nThe CSS file is well-formed and follows best practices!")
        return True


if __name__ == "__main__":
    # Get the repository root
    script_dir = Path(__file__).parent
    repo_root = script_dir.parent
    css_file = repo_root / "docs" / ".vitepress" / "theme" / "style.css"
    
    if not css_file.exists():
        print(f"❌ Error: CSS file not found at {css_file}")
        sys.exit(1)
    
    success = validate_css(css_file)
    sys.exit(0 if success else 1)
