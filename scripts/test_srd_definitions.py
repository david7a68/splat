"""Source and failure-boundary tests for the full catalogue pass."""
import collections
import json
import unittest
from pathlib import Path
import yaml
from convert_srd import load_note
from convert_srd_definitions import (SOURCE, OUTPUT, CONTRACTS, spell_fields,
                                    creature_fields, equipment_tables, progression_tables)


class DefinitionTests(unittest.TestCase):
    def test_ammunition_excerpt_excludes_unrelated_gear_and_keeps_its_table(self):
        import subprocess
        import tempfile
        from import_srd import positioned_lines
        from convert_srd_definitions import section_sources
        with tempfile.TemporaryDirectory() as scratch:
            xml = Path(scratch)/'source.xml'
            subprocess.run(['pdftohtml','-xml','-hidden','-i',str(SOURCE/'source.pdf'),str(xml)],
                           check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            sources = {s['slug']: s for s in section_sources(positioned_lines(xml.read_text()))}
        ammunition = sources['ammunition_varies']
        self.assertNotIn('Spyglass', ammunition['body'])
        self.assertNotIn('Adventuring Gear', ammunition['body'])
        self.assertIn('storage must be bought separately', ammunition['body'])
        self.assertIn('| Bullets, Sling | 20 | Pouch | 1½ lb. | 4 CP |', ammunition['body'])
        self.assertEqual(ammunition['pages'], [94,96])
        gear = sources['adventuring_gear']
        self.assertIn('| Spyglass | 1 lb. | 1,000 GP |', gear['body'])
        self.assertEqual(gear['pages'], [94,95])
        # Every line of the inserted table belongs to its overview, once.
        p95 = [line for source in sources.values() for line in source['positioned_lines'] if line['page']==95]
        self.assertEqual(p95, [line for line in gear['positioned_lines'] if line['page']==95])
        self.assertEqual(len(ammunition['line_spans']), 2)
        addresses = []
        for source in sources.values():
            spans = source['line_spans'] if 'line_spans' in source else [source['line_span']]
            self.assertEqual(sum(span['count'] for span in spans), len(source['positioned_lines']))
            addresses.extend(i for span in spans for i in range(span['first'], span['first']+span['count']))
        self.assertEqual(sorted(addresses), list(range(len(addresses))))

    def test_every_spell_header_is_converted_from_official_source(self):
        spells=list((SOURCE/'spell').glob('*.md'))
        self.assertEqual(len(spells),339)
        for path in spells:
            with self.subTest(path=path):
                _,body=load_note(path)
                spell=spell_fields(body)['splat_spell']
                self.assertIn(spell['level'],range(10))
                self.assertTrue(spell['components'])
        _,body=load_note(SOURCE/'spell/searing_smite.md')
        spell=spell_fields(body)['splat_spell']
        self.assertFalse(spell['concentration'])
        self.assertIn('immediately after hitting',spell['casting_time'])
        self.assertEqual(spell['duration'],'1 minute')

    def test_variable_hit_points_and_supplied_unsigned_save(self):
        _,body=load_note(SOURCE/'creature/animated_object.md')
        fields=creature_fields(body)
        self.assertNotIn('max_hp',fields['splat_stats'])
        _,body=load_note(SOURCE/'creature/young_white_dragon.md')
        fields=creature_fields(body)
        self.assertEqual(fields['splat_stats']['max_hp']['value'],123)
        self.assertEqual(fields['splat_stats']['intelligence']['value'],6)
        self.assertEqual(fields['splat_stats']['save_intelligence'],{'value':2})
        self.assertNotIn('ambiguous_saves',fields['splat_creature'])

    def test_weapon_and_armor_tables_preserve_special_cases(self):
        rows={r['name']:r['fields']['splat_item'] for r in equipment_tables()}
        self.assertEqual(len(rows),51)
        self.assertEqual(rows['Dart']['mass'],{'value':0.25,'unit':'lb'})
        self.assertIn('unless mounted',rows['Lance']['properties_text'])
        self.assertIn('Two-Handed',rows['Heavy Crossbow']['properties_text'])
        self.assertEqual(rows['Plate Armor']['armor']['base_ac'],18)
        self.assertEqual(rows['Plate Armor']['armor']['strength_minimum'],15)
        self.assertEqual(rows['Shield']['armor']['ac_bonus'],2)
        self.assertEqual(rows['Half Plate Armor']['armor']['dexterity_modifier_max'],2)

    def test_all_class_levels_and_distinct_slot_models(self):
        tables={r['name']:r['fields']['splat_rule']['parameters']['rows'] for r in progression_tables()}
        self.assertEqual(len(tables),12)
        self.assertTrue(all(len(t)==20 for t in tables.values()))
        self.assertEqual(tables['Wizard Progression'][19]['spell_slots_9'],1)
        self.assertEqual(tables['Warlock Progression'][19]['pact_slots'],4)
        self.assertEqual(tables['Warlock Progression'][19]['pact_slot_level'],5)
        self.assertEqual(tables['Barbarian Progression'][0]['rages'],2)
        self.assertEqual(tables['Barbarian Progression'][19]['rage_damage'],4)
        self.assertEqual(tables['Monk Progression'][1]['unarmored_movement'],10)
        self.assertEqual(tables['Rogue Progression'][19]['sneak_attack'],'10d6')

    def test_revised_complex_contracts_preserve_rule_outcomes(self):
        specs=yaml.safe_load(CONTRACTS.read_text())
        counter=specs['dnd2024:spells/counterspell']['fields']['splat_actions']['cast']['steps'][0]['parameters']
        self.assertEqual(counter['save_ability'],'constitution')
        self.assertEqual(counter['failure']['spell_slot'],'not_expended')
        poly=specs['dnd2024:spells/polymorph']['fields']['splat_actions']['cast']['steps'][0]['parameters']
        self.assertIn('hit_points',poly['retained'])
        self.assertEqual(poly['temporary_hp'],'form_hit_points')
        hydra=specs['dnd2024:creatures/hydra']['fields']['splat_traits']['heads']['parameters']
        self.assertEqual(hydra['reaction_capacity_recomputed_at'],'owner_turn_start')
        self.assertEqual(hydra['healing_per_regrowth_event'],20)
        bag=specs['dnd2024:items/bag_of_holding']['fields']['splat_item']
        self.assertEqual(bag['containment']['capacity']['volume']['value'],64)
        self.assertEqual(bag['containment']['carried_mass']['value'],5)

    def test_catalogue_keeps_all_named_sources_and_separate_gap_classes(self):
        report=json.loads((OUTPUT/'coverage.json').read_text())
        records=report['entries']
        self.assertEqual(len({r['key'] for r in records}),len(records))
        self.assertEqual(len({r['id'] for r in records}),len(records))
        represented={r['source'] for r in records}
        for category in ['creature','spell','magic_item','glossary']:
            for path in (SOURCE/category).glob('*.md'):
                meta,_=load_note(path)
                self.assertIn(meta['splat_key'],represented)
        counts=collections.Counter(r['category'] for r in records)
        for category in ['class','subclass','table']:self.assertEqual(counts[category],12)
        for r in records:
            self.assertEqual(r['status']=='partial',bool(r['conversion_gaps']))
        self.assertEqual(report['counts']['executable_full_definitions'],0)


class CompletedContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        from srd_contracts import read_research
        cls.research = read_research()

    def parameters(self, key):
        return self.research[key]['fields']['splat_rule']['parameters']

    def test_full_pass_has_no_missing_research_or_conversion_gaps(self):
        report=json.loads((OUTPUT/'coverage.json').read_text())
        self.assertEqual(set(self.research),{entry['key'] for entry in report['entries']})
        self.assertEqual(report['counts']['specified'],1970)
        self.assertEqual(report['counts']['informational'],95)
        self.assertEqual(report['counts']['partial'],0)
        self.assertTrue(all(not entry['conversion_gaps'] for entry in report['entries']))
        registry=json.loads((OUTPUT/'contracts.json').read_text())['contracts']
        self.assertEqual(len(registry),1970)
        self.assertTrue(all(not contract['implemented'] for contract in registry))

    def test_independent_audit_corrections_are_encoded(self):
        apparatus=self.parameters('dnd2024:items/apparatus_of_the_crab')
        self.assertEqual(apparatus['levers'][4]['up']['attack']['hit']['damage']['roll']['scope'],'attack')
        wonder=self.parameters('dnd2024:items/wand_of_wonder')
        repeat=wonder['effects'][17]['effect']['save']['failure']['repeat_save']
        self.assertEqual(repeat['requires'],{'condition':'restrained','source':'this_effect','active':True})

    def test_revision_specific_rules_are_preserved_in_final_contracts(self):
        counter=self.parameters('dnd2024:spells/counterspell')
        self.assertEqual(counter['save']['ability'],'constitution')
        self.assertEqual(counter['failure']['slot_cast']['spell_slot'],'not_expended')
        poly=self.parameters('dnd2024:spells/polymorph')
        self.assertIn('hit_points',poly['form']['retain'])
        self.assertTrue(poly['temporary_hp']['remainder_vanishes_on_spell_end'])
        heads=self.parameters('dnd2024:creatures/hydra')['features']
        self.assertEqual(heads['multiple_heads']['regrow']['heal_per_regrowth_event'],20)
        self.assertTrue(heads['reactive_heads']['recompute_only_at_refresh'])
        self.assertEqual(heads['reactive_heads']['refresh_event'],'self.turn_start')

    def test_contract_shapes_and_source_prose_are_not_interchangeable(self):
        from srd_contracts import json_shape, check_shape, UniqueLoader
        shape=json_shape({'charges':7,'consumed':False})
        check_shape({'charges':4,'consumed':True},shape)
        with self.assertRaises(ValueError):check_shape({'charges':'four','consumed':True},shape)
        with self.assertRaises(ValueError):check_shape({'charges':4,'consumed':True,'run_script':'x'},shape)
        with self.assertRaises(ValueError):yaml.load('parameters: {charges: 7, charges: 4}',Loader=UniqueLoader)

    def test_source_case_choices_do_not_get_invented_defaults(self):
        creation=self.parameters('dnd2024:spells/creation')
        self.assertEqual(creation['duration_by_material_seconds']['vegetable'],86400)
        self.assertTrue(creation['unlisted_mineral_duration']['no_automatic_default'])
        self.assertEqual(creation['unlisted_mineral_duration']['required_input'],'material_duration_resolution')



class MarkdownLayoutTests(unittest.TestCase):
    def test_cells_preserve_yaml_types_and_markdown_punctuation(self):
        from markdown_content import cell
        from srd_contracts import UniqueLoader
        import re
        for value in ['true', 'null', '007', 'yes', '', ' text ', 'a|b', 'a\\|b',
                      '`code`', 'line\nbreak', 'a\\b', 'é', 0, -2, False, None,
                      ['one', 'two'], [], {}]:
            with self.subTest(value=value):
                encoded = cell(value)
                fence = re.match(r'`+', encoded)[0]
                payload = encoded[len(fence)+1:-len(fence)-1]
                self.assertEqual(yaml.load(payload, Loader=UniqueLoader), value)
                self.assertNotIn('|', payload)
                self.assertNotIn('\n', payload)

    def test_notes_have_one_body_specification_and_no_embedded_reports(self):
        from markdown_content import IDENTITY, REPORT_FIELDS
        report = json.loads((OUTPUT/'coverage.json').read_text())
        for record in report['entries']:
            meta, body = load_note(OUTPUT/record['path'])
            self.assertLessEqual(meta.keys(), IDENTITY)
            self.assertFalse(meta.keys() & REPORT_FIELDS)
            self.assertIn('## Source text', body)
            self.assertNotIn('splat_clauses', body)
        registry = json.loads((OUTPUT/'contracts.json').read_text())
        self.assertEqual(registry['format'], 2)
        self.assertTrue(all('specification' not in c for c in registry['contracts']))

class MappedTableTests(unittest.TestCase):
    def test_ammunition_variants_and_spell_lists_are_coherent_tables(self):
        ammo=(OUTPUT/'equipment/ammunition_varies.md').read_text()
        self.assertIn('| Variant | Quantity | Storage | Weight | Cost |',ammo)
        self.assertIn('1½ lb',ammo)
        self.assertNotIn('##### Storage',ammo)
        self.assertNotIn('| Type | Amount | Storage | Weight | Cost |',ammo)
        mage=(OUTPUT/'creatures/mage.md').read_text()
        self.assertIn('| Spell | Frequency |',mage)
        self.assertIn('"dnd2024:spells/fireball"',mage)
        healing=(OUTPUT/'items/potions_of_healing.md').read_text()
        self.assertIn('| Name | Rarity | Healing |',healing)
        self.assertIn('10d4+20',healing)
        self.assertNotIn('Potion HP Regained Rarity',healing)
        frost=(OUTPUT/'items/staff_of_frost.md').read_text()
        self.assertNotIn('Spell Cost Spell Cost',frost)
        self.assertIn('Regaining Charges.',frost)

    def test_plan_preserves_null_false_empty_maps_and_partial_objects(self):
        from mapped_tables import plan,remove_path
        rows=[{'id':'a','amount':None,'nested':{'amount':2,'rule':{'choices':[{'x':0}]}},'flag':False},
              {'id':'b','amount':0,'nested':{'amount':4,'rule':{}},'flag':True}]
        mapped=plan('splat_rule/parameters/options',rows)
        amount=next(c for c in mapped['columns'].values() if c.get('path')=='amount')
        self.assertTrue(amount['nullable'])
        self.assertEqual(amount['type'],'integer')
        self.assertEqual(mapped['order'],['a','b'])
        self.assertIn('Details',mapped['columns'])
        other={'value':0}
        self.assertFalse(remove_path(other,'value/unused'))
        self.assertEqual(other,{'value':0})

if __name__=='__main__':unittest.main()
