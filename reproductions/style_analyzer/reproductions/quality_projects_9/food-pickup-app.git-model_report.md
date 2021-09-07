# Model report for file:///tmp/top-repos-quality-repos-bg_8qta_/food-pickup-app.git HEAD 808fbde3c98308cae23ff425f423b69e6b533df9

### Dump

```json
{'created_at': '2021-08-31 21:38:02',
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
 'size': '15.6 kB',
 'tags': [],
 'uuid': '7a052b08-10b9-4948-8602-bac954a0a6c2',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-bg_8qta_/food-pickup-app.git 808fbde3c98308cae23ff425f423b69e6b533df9

# javascript
7 rules, avg.len. 5.0
## train
PPCR: 0.783033
### report
macro
{'f1-score': 0.47386556245117895,
 'precision': 0.49993656042787726,
 'recall': 0.4542653143711198,
 'support': 4652}
micro
{'f1-score': 0.8753224419604471,
 'precision': 0.8753224419604472,
 'recall': 0.8753224419604472,
 'support': 4652}
weighted
{'f1-score': 0.8591087908414429,
 'precision': 0.8513424485734242,
 'recall': 0.8753224419604472,
 'support': 4652}
### report_full
macro
{'f1-score': 0.44983463536122265,
 'precision': 0.49993656042787726,
 'recall': 0.41677209633859696,
 'support': 5941}
micro
{'f1-score': 0.7688095912394978,
 'precision': 0.8753224419604472,
 'recall': 0.6854064972226898,
 'support': 5941}
weighted
{'f1-score': 0.6856654679968188,
 'precision': 0.7001147276855677,
 'recall': 0.6854064972226898,
 'support': 5941}
## test
PPCR: 0.816972
### report
macro
{'f1-score': 0.3856333691582766,
 'precision': 0.4116673709079907,
 'recall': 0.3766922175820824,
 'support': 982}
micro
{'f1-score': 0.8156822810590632,
 'precision': 0.8156822810590632,
 'recall': 0.8156822810590632,
 'support': 982}
weighted
{'f1-score': 0.798888196145141,
 'precision': 0.808297242814401,
 'recall': 0.8156822810590632,
 'support': 982}
### report_full
macro
{'f1-score': 0.3663711354055132,
 'precision': 0.4116673709079907,
 'recall': 0.3425799154328807,
 'support': 1202}
micro
{'f1-score': 0.7335164835164835,
 'precision': 0.8156822810590632,
 'recall': 0.6663893510815307,
 'support': 1202}
weighted
{'f1-score': 0.6609578537264055,
 'precision': 0.6815522434295528,
 'recall': 0.6663893510815307,
 'support': 1202}
```

## javascript
### Summary
1 rules, avg.len. 4.0

| | |
|-|-|
|Min support|149|
|Max support|149|
|Min confidence|0.9765100479125977|
|Max confidence|0.9765100479125977|

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
                     'min_samples_split': 227,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.977. Support: 149.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.0, "max_conf": 0.9765100479125977, "max_support": 149, "min_conf": 0.9765100479125977, "min_support": 149, "num_rules": 1}}
```
</details>
