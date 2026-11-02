---
splat_schema: 1
splat_id: cece6c90-0039-55f7-83d9-803e352b05c7
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_choose_a_species
splat_name: Choose a Species
splat_origin:
  key: dnd2024:reference/pages/020
splat_sources:
- title: SRD 5.2.1
  locator: pp. 20
  url: https://www.dndbeyond.com/srd
---

# Choose a Species

## Source text

<!-- SRD page 20 -->

Choose a species for your character. The following species options are detailed in “Character Origins”: Dragonborn, Dwarf, Elf, Gnome, Goliath, Halfling, Human, Orc, and Tiefling. Once you’ve chosen a species, write it on your character sheet. Then record your species’ traits.

Your character’s size and Speed are determined by the character’s species; record these in the appropriate places on your character sheet as well (you may write just the first letter of your size).

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/create_your_character_choose_a_species ` |
| ` implementation ` | ` dnd2024:rule_create_your_character_choose_a_species ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` inherit ` | ` ["traits", "size", "speed"] ` |

#### Choose

<!-- splat:yaml path=splat_rule/parameters/choose -->

```yaml
count: 1
options:
- key: dnd2024:species/dragonborn
- key: dnd2024:species/dwarf
- key: dnd2024:species/elf
- key: dnd2024:species/gnome
- key: dnd2024:species/goliath
- key: dnd2024:species/halfling
- key: dnd2024:species/human
- key: dnd2024:species/orc
- key: dnd2024:species/tiefling
```
