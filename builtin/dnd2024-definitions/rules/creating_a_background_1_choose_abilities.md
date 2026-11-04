---
splat_schema: 2
splat_id: 00f66409-316e-5ce6-805a-8f00df17b893
splat_kind: rule
splat_key: dnd2024:rules/creating_a_background_1_choose_abilities
splat_name: '1: Choose Abilities'
splat_origin: dnd2024:reference/pages/192
splat_sources:
- SRD 5.2.1, pp. 192, 193, https://www.dndbeyond.com/srd
---

# 1: Choose Abilities

## Source text

<!-- SRD page 192 -->

Choose three abilities that seem appropriate for the background:

Strength or Dexterity. These abilities are ideal for a background involving physical exertion.

Constitution. This ability is ideal for a background that involves endurance or long hours of activity.

<!-- SRD page 193 -->

Intelligence or Wisdom. One or both abilities are ideal for a background that focuses on cerebral or spiritual matters.

Charisma. This ability is ideal for a background that involves performance or social interaction.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/creating_a_background_1_choose_abilities ` |
| ` implementation ` | ` dnd2024:rule_creating_a_background_1_choose_abilities ` |

### Parameters

#### Background ability choice

<!-- splat:values path=splat_rule/parameters/background_ability_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 3 ` |
| ` distinct ` | ` true ` |
| ` options ` | ` ["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"] ` |
| ` criterion ` | ` appropriate_to_background ` |

#### Interleaved terrain dc columns

<!-- splat:yaml path=splat_rule/parameters/interleaved_terrain_dc_columns -->

```yaml
source:
  key: dnd2024:rules/travel_pace_special_movement
field: terrain_table
```
