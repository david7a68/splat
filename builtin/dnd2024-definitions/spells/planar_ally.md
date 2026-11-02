---
splat_schema: 1
splat_id: c13bbdbf-5ce6-5d07-8de8-3dbf07247a63
splat_kind: spell
splat_key: dnd2024:spells/planar_ally
splat_name: Planar Ally
splat_origin:
  key: dnd2024:reference/spell/planar_ally
splat_sources:
- title: SRD 5.2.1
  locator: pp. 152
  url: https://www.dndbeyond.com/srd
---

# Planar Ally

## Source text

<!-- SRD page 152 -->

Level 6 Conjuration (Cleric)\
Casting Time: 10 minutes\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous

You beseech an otherworldly entity for aid. The being must be known to you: a god, a demon prince, or some other being of cosmic power. That entity sends a Celestial, an Elemental, or a Fiend loyal to it to aid you, making the creature appear in an unoccupied space within range. If you know a specific creature’s name, you can speak that name when you cast this spell to request that creature, though you might get a different creature anyway (GM’s choice).

When the creature appears, it is under no compulsion to behave a particular way. You can ask it to perform a service in exchange for payment, but it isn’t obliged to do so. The requested task could range from simple (fly us across the chasm, or help us fight a battle) to complex (spy on our enemies, or protect us during our foray into the dungeon). You must be able to communicate with the creature to bargain for its services.

Payment can take a variety of forms. A Celestial might require a sizable donation of gold or magic items to an allied temple, while a Fiend might demand a living sacrifice or a gift of treasure. Some creatures might exchange their service for a quest undertaken by you.

A task that can be measured in minutes requires a payment worth 100 GP per minute. A task measured in hours requires 1,000 GP per hour. And a task measured in days (up to 10 days) requires 10,000 GP per day. The GM can adjust these payments based on the circumstances under which you cast the spell. If the task is aligned with the creature’s ethos, the payment might be halved or even waived. Nonhazardous tasks typically require only half the suggested payment, while especially dangerous tasks might require a greater gift. Creatures rarely accept tasks that seem suicidal.

After the creature completes the task, or when the agreed-upon duration of service expires, the creature returns to its home plane after reporting back to you if possible. If you are unable to agree on a price for the creature’s service, the creature immediately returns to its home plane.

Original layout: [[../../srd-5.2.1/pages/152|page 152]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 6 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["cleric"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` 10 minutes ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/planar_ally ` |
| ` implementation ` | ` dnd2024:spell_planar_ally ` |

### Parameters

#### Patron

<!-- splat:values path=splat_rule/parameters/patron -->

| Field | Value |
| --- | --- |
| ` known_to_caster ` | ` true ` |
| ` kind ` | ` cosmic_power_entity ` |

#### Summon

<!-- splat:values path=splat_rule/parameters/summon -->

| Field | Value |
| --- | --- |
| ` creature_types ` | ` ["celestial", "elemental", "fiend"] ` |
| ` loyal_to_patron ` | ` true ` |
| ` destination ` | ` unoccupied_in_range ` |
| ` specific_name_request_optional ` | ` true ` |
| ` final_creature_chooser ` | ` gm ` |

#### Service

<!-- splat:values path=splat_rule/parameters/service -->

| Field | Value |
| --- | --- |
| ` compelled ` | ` false ` |
| ` communication_required_for_bargain ` | ` true ` |
| ` task_choice ` | ` caster ` |
| ` acceptance_choice ` | ` summoned_creature ` |
| ` payment_forms ` | ` ["gold", "magic_items", "sacrifice", "treasure", "quest"] ` |

##### Price rates

<!-- splat:table
path: splat_rule/parameters/service/price_rates
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Unit:
    path: unit
    type: symbol
  Gp:
    path: gp
    type: integer
  Maximum duration days:
    path: maximum_duration_days
    type: integer
    optional: true
-->

| Key | Unit | Gp | Maximum duration days |
| --- | --- | --- | --- |
| ` row_1 ` | ` Minute ` | ` 100 ` |  |
| ` row_2 ` | ` Hour ` | ` 1000 ` |  |
| ` row_3 ` | ` Day ` | ` 10000 ` | ` 10 ` |

##### Gm adjustment

<!-- splat:yaml path=splat_rule/parameters/service/gm_adjustment -->

```yaml
decision: price_amount
context: casting_circumstances
ethos_aligned_options:
- half
- waived
- unchanged
nonhazardous_default_fraction:
  numerator: 1
  denominator: 2
especially_dangerous: greater_gift
suicidal_acceptance: rare
```

#### Return home

<!-- splat:values path=splat_rule/parameters/return_home -->

| Field | Value |
| --- | --- |
| ` events ` | ` ["task_completed", "agreed_duration_expired"] ` |
| ` report_to_caster_first_if_possible ` | ` true ` |
| ` destination ` | ` home_plane ` |
| ` no_price_agreement ` | ` immediate_return ` |
