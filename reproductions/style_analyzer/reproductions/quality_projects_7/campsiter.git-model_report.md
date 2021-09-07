# Model report for file:///tmp/top-repos-quality-repos-87w84oth/campsiter.git HEAD 2b0ee376508a4a1f9985c220c4b9798c45293cb7

### Dump

```json
{'created_at': '2021-09-01 02:48:38',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.4 kB',
 'tags': [],
 'uuid': '5192f540-a49e-4c35-a8f6-8bc9cc73896b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-87w84oth/campsiter.git 2b0ee376508a4a1f9985c220c4b9798c45293cb7

# javascript
26 rules, avg.len. 6.4
## train
PPCR: 0.886904
### report
macro
{'f1-score': 0.7599639241335496,
 'precision': 0.7857230893363192,
 'recall': 0.739558263771141,
 'support': 18915}
micro
{'f1-score': 0.9307956648162834,
 'precision': 0.9307956648162834,
 'recall': 0.9307956648162834,
 'support': 18915}
weighted
{'f1-score': 0.9293060279898756,
 'precision': 0.9291945154336949,
 'recall': 0.9307956648162834,
 'support': 18915}
### report_full
macro
{'f1-score': 0.6707282733380644,
 'precision': 0.7857230893363192,
 'recall': 0.6086040593064017,
 'support': 21327}
micro
{'f1-score': 0.8750062124148901,
 'precision': 0.9307956648162834,
 'recall': 0.8255263281286632,
 'support': 21327}
weighted
{'f1-score': 0.8591866613748905,
 'precision': 0.913526126169636,
 'recall': 0.8255263281286632,
 'support': 21327}
## test
PPCR: 0.891106
### report
macro
{'f1-score': 0.7407971475828743,
 'precision': 0.7716407032142099,
 'recall': 0.7208374871813984,
 'support': 3617}
micro
{'f1-score': 0.9353055017970694,
 'precision': 0.9353055017970694,
 'recall': 0.9353055017970694,
 'support': 3617}
weighted
{'f1-score': 0.9331203121214438,
 'precision': 0.9328862363294919,
 'recall': 0.9353055017970694,
 'support': 3617}
### report_full
macro
{'f1-score': 0.6594609959374093,
 'precision': 0.7716407032142099,
 'recall': 0.6060440262739499,
 'support': 4059}
micro
{'f1-score': 0.8814486711829076,
 'precision': 0.9353055017970694,
 'recall': 0.8334565163833456,
 'support': 4059}
weighted
{'f1-score': 0.8630249605984895,
 'precision': 0.9142907374217225,
 'recall': 0.8334565163833456,
 'support': 4059}
```

## javascript
### Summary
12 rules, avg.len. 6.2

| | |
|-|-|
|Min support|94|
|Max support|3177|
|Min confidence|0.9404473900794983|
|Max confidence|0.9979079365730286|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.label in {<space>}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.987. Support: 188.` |
| 2 | `  -1.label not in {<space>}<br>	∧ -1.roles in {FUNCTION}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 224.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {FUNCTION}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 109.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {FUNCTION}<br>	∧ -3.reserved not in {,}<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 1654.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.998. Support: 233.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.998. Support: 239.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 140.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 156.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {, }}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 2180.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 188.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +2.roles not in {BINARY}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 3177.` |
| 12 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +2.roles not in {BINARY}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 94.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.25, "max_conf": 0.9979079365730286, "max_support": 3177, "min_conf": 0.9404473900794983, "min_support": 94, "num_rules": 12}}
```
</details>
