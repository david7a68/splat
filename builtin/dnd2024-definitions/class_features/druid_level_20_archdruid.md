---
splat_schema: 2
splat_id: f0c0deaa-e5fa-524d-9813-d6bffca0b9cc
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_20_archdruid
splat_name: 'Level 20: Archdruid'
splat_origin: dnd2024:reference/pages/043
splat_sources:
- SRD 5.2.1, pp. 43, https://www.dndbeyond.com/srd
---

# Level 20: Archdruid

## Source text

<!-- SRD page 43 -->

The vitality of nature constantly blooms within you, granting you the following benefits.

Evergreen Wild Shape. Whenever you roll Initiative and have no uses of Wild Shape left, you regain one expended use of it.

Nature Magician. You can convert uses of Wild Shape into a spell slot (no action required). Choose a number of your unexpended uses of Wild Shape and convert them into a single spell slot, with each use contributing 2 spell levels. For example, if you convert two uses of Wild Shape, you produce a level 4 spell slot. Once you use this benefit, you can’t do so again until you finish a Long Rest.

Longevity. The primal magic that you wield causes you to age more slowly. For every ten years that pass, your body ages only one year.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/druid_level_20_archdruid ` |
| ` implementation ` | ` dnd2024:class_features_druid_level_20_archdruid ` |

### Parameters

#### Evergreen wild shape

<!-- splat:yaml path=splat_rule/parameters/evergreen_wild_shape -->

```yaml
trigger: initiative.rolled
requires:
  wild_shape_uses: 0
restore_uses: 1
```

#### Nature magician

<!-- splat:yaml path=splat_rule/parameters/nature_magician -->

```yaml
action: none
resource:
  id: archdruid.nature_magician
  maximum: 1
  recovery:
  - event: rest.long_completed
    amount: all
cost_uses: 1
convert:
  resource: druid.wild_shape
  choose_unexpended_count: true
  minimum_use_count: 1
  create_spell_slot_count: 1
  slot_level:
    op: multiply
    args:
    - 2
    - var: chosen_use_count
```

#### Aging

<!-- splat:values path=splat_rule/parameters/aging -->

| Field | Value |
| --- | --- |
| ` body_years_per_elapsed_year ` | ` 0.1 ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` druid ` |
| ` minimum_level ` | ` 20 ` |
