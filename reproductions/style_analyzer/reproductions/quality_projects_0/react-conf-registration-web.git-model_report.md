# Model report for file:///tmp/top-repos-quality-repos-dqfg01ai/react-conf-registration-web.git HEAD 6f3ea2fd8ccd690cfa1d29e5b317b669b97a1e13

### Dump

```json
{'created_at': '2021-09-01 19:49:30',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-74-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.7 kB',
 'tags': [],
 'uuid': 'f9c4024e-ffce-44e5-ad4d-0ab1195727b5',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-dqfg01ai/react-conf-registration-web.git 6f3ea2fd8ccd690cfa1d29e5b317b669b97a1e13

# javascript
29 rules, avg.len. 7.6
## train
PPCR: 0.858248
### report
macro
{'f1-score': 0.9225061654847264,
 'precision': 0.9244495833307477,
 'recall': 0.9230042554650383,
 'support': 27633}
micro
{'f1-score': 0.964824666160026,
 'precision': 0.964824666160026,
 'recall': 0.964824666160026,
 'support': 27633}
weighted
{'f1-score': 0.9644466744943575,
 'precision': 0.9646433073137827,
 'recall': 0.964824666160026,
 'support': 27633}
### report_full
macro
{'f1-score': 0.8293697926162962,
 'precision': 0.9244495833307477,
 'recall': 0.7579046040383558,
 'support': 32197}
micro
{'f1-score': 0.8912251378906905,
 'precision': 0.964824666160026,
 'recall': 0.8280585147684567,
 'support': 32197}
weighted
{'f1-score': 0.88943159702858,
 'precision': 0.9637754423702122,
 'recall': 0.8280585147684567,
 'support': 32197}
## test
PPCR: 0.827192
### report
macro
{'f1-score': 0.9198870718740407,
 'precision': 0.9220584981014637,
 'recall': 0.9187394981669088,
 'support': 3839}
micro
{'f1-score': 0.9460797082573587,
 'precision': 0.9460797082573587,
 'recall': 0.9460797082573587,
 'support': 3839}
weighted
{'f1-score': 0.945587660064886,
 'precision': 0.9460801737212026,
 'recall': 0.9460797082573587,
 'support': 3839}
### report_full
macro
{'f1-score': 0.8181559182440219,
 'precision': 0.9220584981014637,
 'recall': 0.7416063122111444,
 'support': 4641}
micro
{'f1-score': 0.8566037735849057,
 'precision': 0.9460797082573587,
 'recall': 0.7825899590605473,
 'support': 4641}
weighted
{'f1-score': 0.8546837316278204,
 'precision': 0.9445198679711594,
 'recall': 0.7825899590605473,
 'support': 4641}
```

## javascript
### Summary
18 rules, avg.len. 8.3

| | |
|-|-|
|Min support|93|
|Max support|12423|
|Min confidence|0.9434782862663269|
|Max confidence|0.9994897842407227|

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
| 1 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.998. Support: 206.` |
| 2 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = ,<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 104.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -4.diff_line ≥ 1<br>	∧ -5.diff_offset ≤ 14<br>	∧ +1.reserved not in {,}<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 122.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -4.diff_line = 0<br>	∧ +1.reserved not in {,}<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 1845.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.999. Support: 980.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 134.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 318.` |
| 8 | `  •••start_col ≥ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = import<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.986. Support: 177.` |
| 9 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 786.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 243.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.950. Support: 431.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = import<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 215.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, import}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 332.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, import}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -4.length ≤ 2<br>	∧ -5.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 93.` |
| 15 | `  •••start_col ≤ 75<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.label not in {<newline>}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, SWITCH}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 134.` |
| 16 | `  •••start_col ≤ 23<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.label not in {<newline>}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, SWITCH}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 247.` |
| 17 | `  •••start_col ≤ 75<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, ;, import, }}<br>	∧ -2.label not in {<newline>}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -5.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, SWITCH}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 12423.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label not in {<newline>}<br>	∧ -3.diff_offset ≤ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.943. Support: 115.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.333333333333334, "max_conf": 0.9994897842407227, "max_support": 12423, "min_conf": 0.9434782862663269, "min_support": 93, "num_rules": 18}}
```
</details>
