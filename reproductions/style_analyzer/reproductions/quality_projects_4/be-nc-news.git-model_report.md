# Model report for file:///tmp/top-repos-quality-repos-c6etyd31/be-nc-news.git HEAD 2d0d7d666013cfbc44da40e51266ea7f439066f0

### Dump

```json
{'created_at': '2021-09-02 17:05:30',
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
 'uuid': 'e79519a2-9458-4c11-8227-76e3a6980cdf',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-c6etyd31/be-nc-news.git 2d0d7d666013cfbc44da40e51266ea7f439066f0

# javascript
14 rules, avg.len. 4.5
## train
PPCR: 0.969046
### report
macro
{'f1-score': 0.9413677423885878,
 'precision': 0.967097719258895,
 'recall': 0.9200497783787418,
 'support': 16342}
micro
{'f1-score': 0.9481091665646799,
 'precision': 0.9481091665646799,
 'recall': 0.9481091665646799,
 'support': 16342}
weighted
{'f1-score': 0.9473842180822726,
 'precision': 0.9507288186075987,
 'recall': 0.9481091665646799,
 'support': 16342}
### report_full
macro
{'f1-score': 0.9235475043245405,
 'precision': 0.967097719258895,
 'recall': 0.8900127945224386,
 'support': 16864}
micro
{'f1-score': 0.9332048424983436,
 'precision': 0.9481091665646799,
 'recall': 0.9187618595825426,
 'support': 16864}
weighted
{'f1-score': 0.930609307634797,
 'precision': 0.9518830062671646,
 'recall': 0.9187618595825426,
 'support': 16864}
## test
PPCR: 0.908745
### report
macro
{'f1-score': 0.7453259142614732,
 'precision': 0.8226435654353274,
 'recall': 0.6946252081027959,
 'support': 478}
micro
{'f1-score': 0.893305439330544,
 'precision': 0.893305439330544,
 'recall': 0.893305439330544,
 'support': 478}
weighted
{'f1-score': 0.8880065329924027,
 'precision': 0.9097836459440396,
 'recall': 0.893305439330544,
 'support': 478}
### report_full
macro
{'f1-score': 0.7071350352807728,
 'precision': 0.8226435654353274,
 'recall': 0.6458356271801649,
 'support': 526}
micro
{'f1-score': 0.850597609561753,
 'precision': 0.893305439330544,
 'recall': 0.811787072243346,
 'support': 526}
weighted
{'f1-score': 0.8297918917110476,
 'precision': 0.9168979648658759,
 'recall': 0.811787072243346,
 'support': 526}
```

## javascript
### Summary
13 rules, avg.len. 4.5

| | |
|-|-|
|Min support|111|
|Max support|6164|
|Min confidence|0.9226962924003601|
|Max confidence|0.9984177350997925|

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
                     'max_depth': 10,
                     'min_samples_leaf': 90,
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = :<br>⇒ y = ␣<br>Confidence: 0.993. Support: 1456.` |
| 2 | `  -1.reserved = ,<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 301.` |
| 3 | `  -1.reserved = ,<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {INITIALIZATION}<br>⇒ y = ⏎<br>Confidence: 0.973. Support: 1401.` |
| 4 | `  -1.reserved not in {,, :}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +4.reserved = :<br>⇒ y = '<br>Confidence: 0.953. Support: 817.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.roles in {BINARY}<br>⇒ y = '<br>Confidence: 0.968. Support: 791.` |
| 6 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.roles not in {BINARY}<br>⇒ y = "<br>Confidence: 0.979. Support: 364.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -4.diff_col ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.978. Support: 478.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.998. Support: 316.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.996. Support: 115.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>⇒ y = ⏎<br>Confidence: 0.982. Support: 193.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ><br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 167.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ␣<br>Confidence: 0.923. Support: 111.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 6164.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.538461538461538, "max_conf": 0.9984177350997925, "max_support": 6164, "min_conf": 0.9226962924003601, "min_support": 111, "num_rules": 13}}
```
</details>
