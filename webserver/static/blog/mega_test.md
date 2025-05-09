# Comprehensive Markdown Test File

## Headers
# H1
## H2
### H3
#### H4
##### H5
###### H6

## Text Formatting
**Bold text**
*Italic text*
***Bold italic***
~~Strikethrough~~
`Inline code`
Normal^superscript^ and~subscript~

## Lists

### Ordered
1. First item
2. Second item
3. Third item
   1. Nested item
   2. Nested item

### Unordered
- Top level
- Bullet point
  - Nested level
  - Sub-item
    - Double nested

## Blockquotes
> Standard blockquote
> Multiple line
> blockquote

>> Nested blockquote
>> With inner content

## Code Blocks

```javascript
// Fenced code block
function test() {
  console.log("Hello World");
}
```

    // Indented code block
    const example = true;

## Links
[Inline link](https://example.com)
[Reference link][1]
[Empty anchor](#)
Autolink: <https://example.com>
[1]: https://example.com

## Images
![Alt text](https://picsum.photos/200/300 "Title text")
![Broken image](missing-image.jpg)

## Tables
| Header 1     | Header 2     |
|--------------|--------------|
| Row 1 Col 1   | Row 1 Col 2   |
| *Italic*      | **Bold**      |
| `Code`        | ~~Strike~~    |

| Left-aligned | Center-aligned | Right-aligned |
|:-------------|:--------------:|--------------:|
| Content      | Content        | Content       |

## Horizontal Rules
---
***
___

## Footnotes
Here's a sentence with a footnote.[^1]

[^1]: This is the footnote content.

## Escaped Characters
\*Not italic\*
\# Not header

## HTML Entities
&copy;
&rarr;
&#x1F600;

## Task Lists
- [ ] Unchecked item
- [x] Checked item

## Mixed HTML
<kbd>Ctrl</kbd>+<kbd>C</kbd>

## Special Characters
≈ ≠ ≤ ≥ ± → ⇒

<!-- HTML comment -->
