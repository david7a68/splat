---
splat_schema: 2
splat_id: 2d39873f-7dc8-5f51-b6d6-87cef64321af
splat_kind: spell
splat_key: dnd2024:spells/conjure_woodland_beings
splat_name: Conjure Woodland Beings
splat_origin: dnd2024:reference/spell/conjure_woodland_beings
splat_sources:
- SRD 5.2.1, pp. 118, 119, https://www.dndbeyond.com/srd
---

# Conjure Woodland Beings


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Conjuration |
| Classes | Druid, Ranger |
| Casting Time | Action |
| Range |  |
| Components | V, S |
| Duration | Concentration, up to 10 minutes |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 118 -->

Level 4 Conjuration (Druid, Ranger)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Concentration, up to 10 minutes

You conjure nature spirits that flit around you in a 10-foot Emanation for the duration. Whenever the Emanation enters the space of a creature you can see and whenever a creature you can see enters the Emanation or ends its turn there, you can force that creature to make a Wisdom saving throw. The creature takes 5d8 Force damage on a failed save or half as much damage on a successful one. A creature makes this save only once per turn.

<!-- SRD page 119 -->

In addition, you can take the Disengage action as a Bonus Action for the spell’s duration.

Using a Higher-Level Spell Slot. The damage increases by 1d8 for each spell slot level above 4.

Original layout: [[../../srd-5.2.1/pages/118|page 118]], [[../../srd-5.2.1/pages/119|page 119]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/conjure_woodland_beings ` |
| ` implementation ` | ` dnd2024:spell_conjure_woodland_beings ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` triggers ` | ` ["area.moves_into_creature", "area.enter", "turn.end"] ` |
| ` optional ` | ` true ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` emanation ` |
| ` radius_feet ` | ` 10 ` |
| ` origin ` | ` caster ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` each_eligible ` |
| ` visible_to_caster ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
dc:
  binding: save_dc
failure:
  damage:
    amount:
      dice:
        count: 5
        sides: 8
        add: 0
    type: force
success:
  damage_fraction:
    numerator: 1
    denominator: 2
```

#### Limit

<!-- splat:values path=splat_rule/parameters/limit -->

| Field | Value |
| --- | --- |
| ` saves_per_creature_per_turn ` | ` 1 ` |

#### Grant

<!-- splat:values path=splat_rule/parameters/grant -->

| Field | Value |
| --- | --- |
| ` action ` | ` disengage ` |
| ` cost ` | ` bonus_action ` |
| ` duration ` | ` spell ` |

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 4 ` |
| ` damage_dice ` | ` 1 ` |
| ` sides ` | ` 8 ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: self
> ```

