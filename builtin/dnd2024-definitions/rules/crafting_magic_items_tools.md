---
splat_schema: 1
splat_id: 5a50b803-e3c0-5ced-af76-c9d017c48c68
splat_kind: rule
splat_key: dnd2024:rules/crafting_magic_items_tools
splat_name: Tools
splat_origin:
  key: dnd2024:reference/pages/206
splat_sources:
- title: SRD 5.2.1
  locator: pp. 206, 207
  url: https://www.dndbeyond.com/srd
---

# Tools

## Source text

<!-- SRD page 206 -->

The Magic Item Tools table lists which tool is required to make a magic item of each category. You must use the required tool to make an item and have proficiency with that tool. Any assistants must also have proficiency with it. For more information on the tools, see “Equipment.”

<!-- SRD page 207 -->

**Magic Item Tools**

| Item Category | Required Tool |
| --- | --- |
| Armor | Leatherworker’s Tools, Smith’s Tools, or Weaver’s Tools depending on the kind of armor as noted in the tools’ descriptions |
| Potion | Alchemist’s Supplies or Herbalism Kit |
| Ring | Jeweler’s Tools |
| Rod | Woodcarver’s Tools |
| Scroll | Calligrapher’s Supplies |
| Staff | Woodcarver’s Tools |
| Wand | Woodcarver’s Tools |
| Weapon | Leatherworker’s Tools, Smith’s Tools, or Woodcarver’s Tools depending on the kind of weapon as noted in the tools’ descriptions |
| Wondrous Item | Tinker’s Tools or the tool required to make the nonmagical item on which the magic item is based |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/crafting_magic_items_tools ` |
| ` implementation ` | ` dnd2024:rules_crafting_magic_items_tools ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` required_tool_use ` | ` true ` |
| ` creator_proficiency_required ` | ` true ` |
| ` assistant_proficiency_required ` | ` true ` |

#### Category tools

<!-- splat:yaml path=splat_rule/parameters/category_tools -->

```yaml
armor:
  options:
  - leatherworkers_tools
  - smiths_tools
  - weavers_tools
  selection: nonmagical_armor_tool_requirement
potion:
  options:
  - alchemists_supplies
  - herbalism_kit
  selection: any
ring:
  options:
  - jewelers_tools
rod:
  options:
  - woodcarvers_tools
scroll:
  options:
  - calligraphers_supplies
staff:
  options:
  - woodcarvers_tools
wand:
  options:
  - woodcarvers_tools
weapon:
  options:
  - leatherworkers_tools
  - smiths_tools
  - woodcarvers_tools
  selection: nonmagical_weapon_tool_requirement
wondrous_item:
  options:
  - tinkers_tools
  - nonmagical_base_item_required_tool
  selection: any
```
