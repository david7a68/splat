---
splat_schema: 2
splat_id: 2f38c561-bd58-5c2f-b824-080a87441002
splat_kind: spell
splat_key: dnd2024:spells/charm_monster
splat_name: Charm Monster
splat_origin: dnd2024:reference/spell/charm_monster
splat_sources:
- SRD 5.2.1, pp. 114, 115, https://www.dndbeyond.com/srd
---

# Charm Monster


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Enchantment |
| Classes | Bard, Druid, Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range | 30 feet |
| Components | V, S |
| Duration | 1 hour |


## Source text

<!-- SRD page 114 -->

Level 4 Enchantment (Bard, Druid, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: 1 hour

<!-- SRD page 115 -->

One creature you can see within range makes a Wisdom saving throw. It does so with Advantage if you or your allies are fighting it. On a failed save, the target has the Charmed condition until the spell ends or until you or your allies damage it. The Charmed creature is Friendly to you. When the spell ends, the target knows it was Charmed by you.

Using a Higher-Level Spell Slot. You can target one additional creature for each spell slot level above 4.

Original layout: [[../../srd-5.2.1/pages/114|page 114]], [[../../srd-5.2.1/pages/115|page 115]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/charm_monster ` |
| ` implementation ` | ` dnd2024:spell_charm_monster ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 3600 ` |

#### Targets

<!-- splat:yaml path=splat_rule/parameters/targets -->

```yaml
kind: creature
visible: true
range_feet: 30
count:
  base: 1
  increment_per_slot_above: 1
  base_slot: 4
```

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` wisdom ` |
| ` dc ` | ` spell_save_dc ` |
| ` advantage_if ` | ` caster_or_allies_fighting_target ` |
| ` success ` | ` unaffected ` |

#### Failure

<!-- splat:values path=splat_rule/parameters/failure -->

| Field | Value |
| --- | --- |
| ` condition ` | ` charmed ` |
| ` source ` | ` caster ` |
| ` attitude_to_caster ` | ` friendly ` |

#### End target

<!-- splat:values path=splat_rule/parameters/end_target -->

| Field | Value |
| --- | --- |
| ` event ` | ` damage.applied ` |
| ` source ` | ` caster_or_allies ` |
| ` target ` | ` affected_target ` |

#### On effect end

<!-- splat:values path=splat_rule/parameters/on_effect_end -->

| Field | Value |
| --- | --- |
| ` target_knows ` | ` caster_charmed_target ` |
