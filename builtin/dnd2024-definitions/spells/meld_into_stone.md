---
splat_schema: 1
splat_id: 25fed2c9-1bb0-556c-a7a6-58c3d2b8fe78
splat_kind: spell
splat_key: dnd2024:spells/meld_into_stone
splat_name: Meld into Stone
splat_origin:
  key: dnd2024:reference/spell/meld_into_stone
splat_sources:
- title: SRD 5.2.1
  locator: pp. 148
  url: https://www.dndbeyond.com/srd
---

# Meld into Stone

## Source text

<!-- SRD page 148 -->

Level 3 Transmutation (Cleric, Druid, Ranger)\
Casting Time: Action or Ritual\
Range: Touch\
Components: V, S\
Duration: 8 hours

You step into a stone object or surface large enough to fully contain your body, merging yourself and your equipment with the stone for the duration. You must touch the stone to do so. Nothing of your presence remains visible or otherwise detectable by nonmagical senses.

While merged with the stone, you can’t see what occurs outside it, and any Wisdom (Perception) checks you make to hear sounds outside it are made with Disadvantage. You remain aware of the passage of time and can cast spells on yourself while merged in the stone. You can use 5 feet of movement to leave the stone where you entered it, which ends the spell. You otherwise can’t move.

Minor physical damage to the stone doesn’t harm you, but its partial destruction or a change in its shape (to the extent that you no longer fit within it) expels you and deals 6d6 Force damage to you. The stone’s complete destruction (or transmutation into a different substance) expels you and deals 50 Force damage to you. If expelled, you move into an unoccupied space closest to where you first entered and have the Prone condition.

Original layout: [[../../srd-5.2.1/pages/148|page 148]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["cleric", "druid", "ranger"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 8 hours ` |
| ` casting_time ` | ` Action or Ritual ` |
| ` ritual ` | ` true ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/meld_into_stone ` |
| ` implementation ` | ` dnd2024:spell_meld_into_stone ` |

### Parameters

#### Stone

<!-- splat:values path=splat_rule/parameters/stone -->

| Field | Value |
| --- | --- |
| ` kind ` | ` ["object", "surface"] ` |
| ` touch ` | ` true ` |
| ` must_fully_contain_body ` | ` true ` |

#### Merge

<!-- splat:values path=splat_rule/parameters/merge -->

| Field | Value |
| --- | --- |
| ` subject ` | ` caster_and_equipment ` |
| ` nonmagical_detection ` | ` false ` |
| ` outside_vision ` | ` false ` |
| ` outside_hearing_perception_checks ` | ` disadvantage ` |
| ` time_awareness ` | ` true ` |
| ` cast_spells_on_self ` | ` true ` |
| ` movement ` | ` false ` |

#### Exit

<!-- splat:values path=splat_rule/parameters/exit -->

| Field | Value |
| --- | --- |
| ` movement_cost_feet ` | ` 5 ` |
| ` location ` | ` entry_point ` |
| ` ends_spell ` | ` true ` |

#### Stone damage

<!-- splat:yaml path=splat_rule/parameters/stone_damage -->

```yaml
minor: no_harm
partial_destruction_or_shape_change_no_longer_fits:
  expel: true
  damage:
    amount:
      dice:
        count: 6
        sides: 6
        add: 0
    type: force
complete_destruction_or_transmutation:
  expel: true
  damage:
    amount: 50
    type: force
```

#### Expulsion

<!-- splat:values path=splat_rule/parameters/expulsion -->

| Field | Value |
| --- | --- |
| ` destination ` | ` nearest_unoccupied_to_entry ` |
| ` condition ` | ` prone ` |
