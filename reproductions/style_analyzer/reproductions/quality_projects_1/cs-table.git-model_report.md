# Model report for file:///tmp/top-repos-quality-repos-7lawyjuc/cs-table.git HEAD 17226957035448b5b3ad84f54d8dff261d473dbb

### Dump

```json
{'created_at': '2021-09-02 02:58:39',
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
 'size': '15.1 kB',
 'tags': [],
 'uuid': '065ab0f7-66f3-48f4-9100-af0a387df1c7',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-7lawyjuc/cs-table.git 17226957035448b5b3ad84f54d8dff261d473dbb

# javascript
8 rules, avg.len. 3.6
## train
PPCR: 0.824953
### report
macro
{'f1-score': 0.9381068489532431,
 'precision': 0.9580638347316962,
 'recall': 0.9233412622287238,
 'support': 3954}
micro
{'f1-score': 0.9610520991401112,
 'precision': 0.9610520991401112,
 'recall': 0.9610520991401112,
 'support': 3954}
weighted
{'f1-score': 0.9598000503277282,
 'precision': 0.9611000296196434,
 'recall': 0.9610520991401112,
 'support': 3954}
### report_full
macro
{'f1-score': 0.8542897028764705,
 'precision': 0.9580638347316962,
 'recall': 0.7788832193108312,
 'support': 4793}
micro
{'f1-score': 0.8688693266262718,
 'precision': 0.9610520991401112,
 'recall': 0.7928228666805759,
 'support': 4793}
weighted
{'f1-score': 0.8632851950762309,
 'precision': 0.9624080321282915,
 'recall': 0.7928228666805759,
 'support': 4793}
## test
PPCR: 0.563739
### report
macro
{'f1-score': 0.7134894730365966,
 'precision': 0.7579008114990123,
 'recall': 0.8133116883116882,
 'support': 199}
micro
{'f1-score': 0.8693467336683417,
 'precision': 0.8693467336683417,
 'recall': 0.8693467336683417,
 'support': 199}
weighted
{'f1-score': 0.8583675209995304,
 'precision': 0.9302642289766818,
 'recall': 0.8693467336683417,
 'support': 199}
### report_full
macro
{'f1-score': 0.57455912588695,
 'precision': 0.7579008114990123,
 'recall': 0.5290584415584415,
 'support': 353}
micro
{'f1-score': 0.6268115942028986,
 'precision': 0.8693467336683417,
 'recall': 0.49008498583569404,
 'support': 353}
weighted
{'f1-score': 0.5966404013724205,
 'precision': 0.9206201229615868,
 'recall': 0.49008498583569404,
 'support': 353}
```

## javascript
### Summary
5 rules, avg.len. 2.8

| | |
|-|-|
|Min support|139|
|Max support|724|
|Min confidence|0.954081654548645|
|Max confidence|0.9988763928413391|

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
                     'min_samples_leaf': 95,
                     'min_samples_split': 203,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = :<br>⇒ y = ␣<br>Confidence: 0.993. Support: 537.` |
| 2 | `  -1.reserved = ,<br>	∧ +2.reserved = ,<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.975. Support: 139.` |
| 3 | `  -1.reserved = ,<br>	∧ +2.reserved not in {,}<br>⇒ y = ⏎<br>Confidence: 0.954. Support: 490.` |
| 4 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, :}<br>⇒ y = '<br>Confidence: 0.999. Support: 445.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 724.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.8, "max_conf": 0.9988763928413391, "max_support": 724, "min_conf": 0.954081654548645, "min_support": 139, "num_rules": 5}}
```
</details>
