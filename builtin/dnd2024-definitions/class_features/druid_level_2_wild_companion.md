---
splat_schema: 2
splat_id: 5fb5c6b0-bcad-5fd8-9885-88f250f23e14
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_2_wild_companion
splat_name: 'Level 2: Wild Companion'
splat_origin: dnd2024:reference/pages/043
splat_sources:
- SRD 5.2.1, pp. 43, https://www.dndbeyond.com/srd
---

# Level 2: Wild Companion

## Source text

<!-- SRD page 43 -->

You can summon a nature spirit that assumes an animal form to aid you. As a Magic action, you can expend a spell slot or a use of Wild Shape to cast the Find Familiar spell without Material components.

When you cast the spell in this way, the familiar is Fey and disappears when you finish a Long Rest.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/druid_level_2_wild_companion ` |
| ` implementation ` | ` dnd2024:class_features_druid_level_2_wild_companion ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |

#### Cost choice

<!-- splat:table
path: splat_rule/parameters/cost_choice
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Resource:
    path: resource
    type: string
  Amount:
    path: amount
    type: integer
  Minimum level:
    path: minimum_level
    type: integer
    optional: true
-->

| Key | Resource | Amount | Minimum level |
| --- | --- | --- | --- |
| ` row_1 ` | ` spell_slot ` | ` 1 ` | ` 1 ` |
| ` row_2 ` | ` druid.wild_shape ` | ` 1 ` |  |

#### Cast

<!-- splat:yaml path=splat_rule/parameters/cast -->

```yaml
spell:
  key: dnd2024:spells/find_familiar
material_components: false
creature_type: fey
familiar_ends: actor.rest.long_completed
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` druid ` |
| ` minimum_level ` | ` 2 ` |
