---
splat_schema: 2
splat_id: 7e0b22bc-9ed8-50fa-b68d-9d1f0718f6bb
splat_kind: spell
splat_key: dnd2024:spells/charm_person
splat_name: Charm Person
splat_origin: dnd2024:reference/spell/charm_person
splat_sources:
- SRD 5.2.1, pp. 115, https://www.dndbeyond.com/srd
---

# Charm Person


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Enchantment |
| Classes | Bard, Druid, Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range | 30 feet |
| Components | V, S |
| Duration | 1 hour |


## Source text

<!-- SRD page 115 -->

Level 1 Enchantment (Bard, Druid, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: 1 hour

One Humanoid you can see within range makes a Wisdom saving throw. It does so with Advantage if you or your allies are fighting it. On a failed save, the target has the Charmed condition until the spell ends or until you or your allies damage it. The Charmed creature is Friendly to you. When the spell ends, the target knows it was Charmed by you.

Using a Higher-Level Spell Slot. You can target one additional creature for each spell slot level above 1.

Original layout: [[../../srd-5.2.1/pages/115|page 115]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/charm_person ` |
| ` implementation ` | ` dnd2024:spell_charm_person ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` save_advantage_if ` | ` caster_or_allies_fighting_target ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` humanoid ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
dc:
  binding: save_dc
failure:
  condition: charmed
  attitude_to_caster: friendly
success:
  effect: none
```

#### End on

<!-- splat:values path=splat_rule/parameters/end_on -->

| Field | Value |
| --- | --- |
| ` event ` | ` damage.applied ` |
| ` source ` | ` ["caster", "caster_allies"] ` |
| ` subject ` | ` target ` |

#### On end

<!-- splat:values path=splat_rule/parameters/on_end -->

| Field | Value |
| --- | --- |
| ` target_knows_charmed_by_caster ` | ` true ` |

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 1 ` |
| ` additional_targets ` | ` 1 ` |
