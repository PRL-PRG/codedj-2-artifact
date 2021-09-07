# Model report for file:///tmp/top-repos-quality-repos-v8ugd2xp/buddy-app-be.git HEAD 66d503f27d1442029f021bba3b15a4d7573daed2

### Dump

```json
{'created_at': '2021-09-02 00:30:14',
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
 'size': '16.1 kB',
 'tags': [],
 'uuid': '3aa5bf2f-207f-4a26-9218-1a020ff7041f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-v8ugd2xp/buddy-app-be.git 66d503f27d1442029f021bba3b15a4d7573daed2

# javascript
35 rules, avg.len. 7.2
## train
PPCR: 0.810886
### report
macro
{'f1-score': 0.4643563231970535,
 'precision': 0.48337833446479156,
 'recall': 0.45603666753486916,
 'support': 4931}
micro
{'f1-score': 0.8099776921516932,
 'precision': 0.8099776921516934,
 'recall': 0.8099776921516934,
 'support': 4931}
weighted
{'f1-score': 0.7758522113312584,
 'precision': 0.7618006430791425,
 'recall': 0.8099776921516934,
 'support': 4931}
### report_full
macro
{'f1-score': 0.3975485611357355,
 'precision': 0.48337833446479156,
 'recall': 0.35733396773666504,
 'support': 6081}
micro
{'f1-score': 0.7253904831093352,
 'precision': 0.8099776921516934,
 'recall': 0.6567998684426903,
 'support': 6081}
weighted
{'f1-score': 0.6707764972027784,
 'precision': 0.7364316192319569,
 'recall': 0.6567998684426903,
 'support': 6081}
## test
PPCR: 0.827539
### report
macro
{'f1-score': 0.4882466495036777,
 'precision': 0.49471406539232887,
 'recall': 0.48349743445049853,
 'support': 1214}
micro
{'f1-score': 0.8583196046128501,
 'precision': 0.8583196046128501,
 'recall': 0.8583196046128501,
 'support': 1214}
weighted
{'f1-score': 0.8410581276502186,
 'precision': 0.8274406260246299,
 'recall': 0.8583196046128501,
 'support': 1214}
### report_full
macro
{'f1-score': 0.4245698171589788,
 'precision': 0.49471406539232887,
 'recall': 0.38488535206791424,
 'support': 1467}
micro
{'f1-score': 0.7773218948153673,
 'precision': 0.8583196046128501,
 'recall': 0.7102931152010906,
 'support': 1467}
weighted
{'f1-score': 0.7332866029844769,
 'precision': 0.777048296922033,
 'recall': 0.7102931152010906,
 'support': 1467}
```

## javascript
### Summary
20 rules, avg.len. 6.6

| | |
|-|-|
|Min support|147|
|Max support|1454|
|Min confidence|0.9224770665168762|
|Max confidence|0.9985207319259644|

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
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.998. Support: 294.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 1454.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 170.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 152.` |
| 5 | `  •••start_col ≤ 16<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 209.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 161.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 1342.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.reserved not in {"}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 147.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.reserved not in {"}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 184.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 1414.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION}<br>	∧ ^2.roles in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 216.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 1090.` |
| 13 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>	∧ ^1.roles in {IDENTIFIER} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 545.` |
| 14 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 338.` |
| 15 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 228.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 150.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.962. Support: 198.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 210.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 179.` |
| 20 | `  •••start_col ≤ 16<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 190.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.6, "max_conf": 0.9985207319259644, "max_support": 1454, "min_conf": 0.9224770665168762, "min_support": 147, "num_rules": 20}}
```
</details>
