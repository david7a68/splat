---
splat_schema: 1
splat_id: c82932e3-e0cc-5ed7-8182-ba9da368c0ff
splat_kind: rule
splat_key: dnd2024:class_features/bard_level_2_jack_of_all_trades
splat_name: 'Level 2: Jack of All Trades'
splat_origin:
  key: dnd2024:reference/pages/032
splat_sources:
- title: SRD 5.2.1
  locator: pp. 32
  url: https://www.dndbeyond.com/srd
---

# Level 2: Jack of All Trades

## Source text

<!-- SRD page 32 -->

You can add half your Proficiency Bonus (round\
down) to any ability check you make that uses a\
skill proficiency you lack and that doesn’t otherwise\
use your Proficiency Bonus.\
For example, if you make a Strength (Athletics)\
check and lack Athletics proficiency, you can add\
half your Proficiency Bonus to the check.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/bard_level_2_jack_of_all_trades ` |
| ` implementation ` | ` dnd2024:class_features_bard_level_2_jack_of_all_trades ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` roll ` | ` ability_check ` |
| ` requires_skill ` | ` true ` |
| ` skill_proficiency ` | ` false ` |
| ` already_uses_proficiency_bonus ` | ` false ` |

#### Optional bonus

<!-- splat:yaml path=splat_rule/parameters/optional_bonus -->

```yaml
op: floor
args:
- op: divide
  args:
  - stat: proficiency_bonus
  - 2
```
