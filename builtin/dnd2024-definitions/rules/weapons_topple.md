---
splat_schema: 1
splat_id: aa5e05a3-e3ac-5ecd-8433-7f5cc3f02807
splat_kind: rule
splat_key: dnd2024:rules/weapons_topple
splat_name: Topple
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
---

# Topple

## Source text

<!-- SRD page 90 -->

If you hit a creature with this weapon, you can force\
the creature to make a Constitution saving throw\
(DC 8 plus the ability modifier used to make the\
attack roll and your Proficiency Bonus). On a failed\
save, the creature has the Prone condition.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/weapons_topple ` |
| ` implementation ` | ` dnd2024:rules_weapons_topple ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` weapon_hit ` |
| ` optional ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
dc:
  op: add
  args:
  - 8
  - var: attack.ability_modifier
  - var: subject.proficiency_bonus
failure:
  condition: prone
```
