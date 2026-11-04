---
splat_schema: 2
splat_id: 3172a067-3138-5c24-9631-0ace78c13094
splat_kind: rule
splat_key: dnd2024:glossary/initiative
splat_name: Initiative
splat_origin: dnd2024:reference/glossary/initiative
splat_sources:
- SRD 5.2.1, pp. 184, https://www.dndbeyond.com/srd
---

# Initiative

## Source text

<!-- SRD page 184 -->

Initiative determines the order of turns during combat. The combat rules in “Playing the Game” explain how to roll Initiative.

Sometimes a GM might have combatants use their Initiative scores instead of rolling Initiative. Your Initiative score equals 10 plus your Dexterity modifier. If you have Advantage on Initiative rolls, increase your Initiative score by 5. If you have Disadvantage on those rolls, decrease that score by 5. See also “Playing the Game” (“Combat”).

Original layout: [[../../srd-5.2.1/pages/184|page 184]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/initiative ` |
| ` implementation ` | ` dnd2024:glossary_initiative ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` purpose ` | ` combat_turn_order ` |

#### Mode choice

<!-- splat:values path=splat_rule/parameters/mode_choice -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` gm ` |
| ` options ` | ` ["roll", "score"] ` |
| ` default ` | ` roll ` |

#### Score

<!-- splat:yaml path=splat_rule/parameters/score -->

```yaml
op: add
args:
- 10
- var: dexterity_modifier
```

#### Score roll mode adjustment

<!-- splat:values path=splat_rule/parameters/score_roll_mode_adjustment -->

| Field | Value |
| --- | --- |
| ` advantage ` | ` 5 ` |
| ` disadvantage ` | ` -5 ` |
| ` normal ` | ` 0 ` |
