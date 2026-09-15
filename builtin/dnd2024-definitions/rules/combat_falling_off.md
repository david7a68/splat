---
splat_schema: 1
splat_id: bddaacc1-13c0-5d9c-9fbc-38adfae6d250
splat_kind: rule
splat_key: dnd2024:rules/combat_falling_off
splat_name: Falling Off
splat_origin:
  key: dnd2024:reference/pages/016
splat_sources:
- title: SRD 5.2.1
  locator: pp. 16
  url: https://www.dndbeyond.com/srd
---

# Falling Off

## Source text

<!-- SRD page 16 -->

If an effect is about to move your mount against its\
will while you’re on it, you must succeed on a DC 10\
Dexterity saving throw or fall off, landing with the\
Prone condition (see “Rules Glossary”) in an unoc-\
cupied space within 5 feet of the mount.\
While mounted, you must make the same save if\
you’re knocked Prone or the mount is.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_falling_off ` |
| ` implementation ` | ` dnd2024:rules_combat_falling_off ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` triggers ` | ` ["mount_forced_movement_pending", "rider_knocked_prone", "mount_knocked_prone"] ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` dexterity ` |
| ` dc ` | ` 10 ` |

#### Failure

<!-- splat:yaml path=splat_rule/parameters/failure -->

```yaml
dismount: true
condition: prone
destination:
  unoccupied: true
  within_mount_feet: 5
```
