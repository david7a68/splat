---
splat_schema: 2
splat_id: 96d4c5ca-6a2d-42a4-8cf7-9083f03f7bb2
splat_kind: container
splat_key: "my_homebrew:containers/supply_pack"
splat_name: Supply Pack
---

A reusable starting setup. Each placement creates its own pack, pouch and
draught stack. This root table is the pack's contents, not an unowned inventory.

<!-- splat:inventory -->

| Key | Item | Quantity | Equipment |
| --- | --- | --- | --- |
| pouch | `my_homebrew:items/pouch` | 1 | stored |

<!-- splat:inventory container=pouch -->

| Key | Item | Quantity | Equipment |
| --- | --- | --- | --- |
| reserve | `my_homebrew:items/healing_draught` | 2 | stored |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item -->
> 
> ```yaml
> key: my_homebrew:items/backpack
> ```

