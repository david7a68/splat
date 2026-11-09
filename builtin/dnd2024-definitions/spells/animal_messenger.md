---
splat_schema: 2
splat_id: 09a6df3c-5bd5-5498-8636-eb626e962461
splat_kind: spell
splat_key: dnd2024:spells/animal_messenger
splat_name: Animal Messenger
splat_origin: dnd2024:reference/spell/animal_messenger
splat_sources:
- SRD 5.2.1, pp. 108, https://www.dndbeyond.com/srd
---

# Animal Messenger


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Enchantment |
| Classes | Bard, Druid, Ranger |
| Casting Time | Action or Ritual |
| Range | 30 feet |
| Components | V, S, M (a morsel of food) |
| Duration | 24 hours |
| Ritual | Yes |
| Concentration | No |


## Source text

<!-- SRD page 108 -->

Level 2 Enchantment (Bard, Druid, Ranger)\
Casting Time: Action or Ritual\
Range: 30 feet\
Components: V, S, M (a morsel of food)\
Duration: 24 hours

A Tiny Beast of your choice that you can see within range must succeed on a Charisma saving throw, or it attempts to deliver a message for you (if the target’s Challenge Rating isn’t 0, it automatically succeeds). You specify a location you have visited and a recipient who matches a general description, such as “a person dressed in the uniform of the town guard” or “a red-haired dwarf wearing a pointed hat.” You also communicate a message of up to twenty-five words. The Beast travels for the duration toward the specified location, covering about 25 miles per 24 hours or 50 miles if the Beast can fly.

When the Beast arrives, it delivers your message to the creature that you described, mimicking your communication. If the Beast doesn’t reach its destination before the spell ends, the message is lost, and the Beast returns to where you cast the spell.

Using a Higher-Level Spell Slot. The spell’s duration increases by 48 hours for each spell slot level above 2.

Original layout: [[../../srd-5.2.1/pages/108|page 108]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/animal_messenger ` |
| ` implementation ` | ` dnd2024:spell_animal_messenger ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |
| ` visible ` | ` true ` |
| ` size ` | ` tiny ` |
| ` creature_type ` | ` beast ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: charisma
dc:
  binding: save_dc
failure:
  effect: deliver_message
success:
  effect: none
```

#### Automatic save success

<!-- splat:values path=splat_rule/parameters/automatic_save_success -->

| Field | Value |
| --- | --- |
| ` challenge_rating_not_equal ` | ` 0 ` |

#### Message

<!-- splat:yaml path=splat_rule/parameters/message -->

```yaml
word_limit: 25
destination:
  visited_by_caster: true
recipient:
  match: caster_general_description
delivery: mimic_caster_communication
```

#### Travel

<!-- splat:yaml path=splat_rule/parameters/travel -->

```yaml
duration_hours:
  op: add
  args:
  - 24
  - op: multiply
    args:
    - 48
    - op: subtract
      args:
      - binding: cast_level
      - 2
miles_per_24_hours:
  flying: 50
  otherwise: 25
rate_precision: approximate
on_arrival: deliver_to_matching_creature
on_duration_expired_before_arrival:
  message: lost
  beast_destination: casting_location
on_arrival_duration_remaining: spell_duration_remaining
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

