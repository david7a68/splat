---
splat_schema: 2
splat_id: ad572d3b-16e7-5478-a402-ec28095e3c6e
splat_kind: item
splat_key: dnd2024:equipment/other_tools_musical_instrument_varies
splat_name: Musical Instrument (Varies)
splat_origin: dnd2024:reference/pages/094
splat_sources:
- SRD 5.2.1, pp. 94, https://www.dndbeyond.com/srd
---

# Musical Instrument (Varies)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 94 -->

Ability: Charisma Weight: Varies\
Utilize: Play a known tune (DC 10), or improvise a song (DC 15)\
Variants: Bagpipes (30 GP, 6 lb.), drum (6 GP, 3 lb.), dulcimer (25 GP, 10 lb.), flute (2 GP, 1 lb.), horn (3 GP, 2 lb.), lute (35 GP, 2 lb.), lyre (30 GP, 2 lb.), pan flute (12 GP, 2 lb.), shawm (2 GP, 1 lb.), viol (30 GP, 1 lb.)





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/other_tools_musical_instrument_varies ` |
| ` implementation ` | ` dnd2024:equipment_other_tools_musical_instrument_varies ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: charisma
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: play_known_tune
  check:
    ability: charisma
    dc: 10
- action: utilize
  operation: improvise_song
  check:
    ability: charisma
    dc: 15
craftable: []
monster_proficient_if_tool_in_stat_block: true
proficiency_rule:
  key: dnd2024:rules/tools_tool_proficiency
crafting_rule:
  key: dnd2024:rules/crafting_nonmagical_items_tools
```

#### Variants

<!-- splat:table
path: splat_rule/parameters/variants
key: Variant
order:
- bagpipes
- drum
- dulcimer
- flute
- horn
- lute
- lyre
- pan_flute
- shawm
- viol
columns:
  Variant:
    path: id
    type: symbol
  Cost:
    path: price
    type: quantity
  Weight:
    path: mass
    type: quantity
-->

| Variant | Cost | Weight |
| --- | --- | --- |
| ` Bagpipes ` | ` 30 GP ` | ` 6 lb ` |
| ` Drum ` | ` 6 GP ` | ` 3 lb ` |
| ` Dulcimer ` | ` 25 GP ` | ` 10 lb ` |
| ` Flute ` | ` 2 GP ` | ` 1 lb ` |
| ` Horn ` | ` 3 GP ` | ` 2 lb ` |
| ` Lute ` | ` 35 GP ` | ` 2 lb ` |
| ` Lyre ` | ` 30 GP ` | ` 2 lb ` |
| ` Pan flute ` | ` 12 GP ` | ` 2 lb ` |
| ` Shawm ` | ` 2 GP ` | ` 1 lb ` |
| ` Viol ` | ` 30 GP ` | ` 1 lb ` |
