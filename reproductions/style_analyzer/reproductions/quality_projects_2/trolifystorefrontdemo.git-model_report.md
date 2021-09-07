# Model report for file:///tmp/top-repos-quality-repos-x6932c50/trolifystorefrontdemo.git HEAD 33759aaf93dc5ea02a24832affa82f81d4189749

### Dump

```json
{'created_at': '2021-09-02 05:08:05',
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
 'size': '16.5 kB',
 'tags': [],
 'uuid': '1ea83589-c598-409d-97fa-f8a0ef351e07',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-x6932c50/trolifystorefrontdemo.git 33759aaf93dc5ea02a24832affa82f81d4189749

# javascript
18 rules, avg.len. 5.9
## train
PPCR: 0.872864
### report
macro
{'f1-score': 0.9009748029959642,
 'precision': 0.9080358767273333,
 'recall': 0.904764337101078,
 'support': 10216}
micro
{'f1-score': 0.9437157400156617,
 'precision': 0.9437157400156617,
 'recall': 0.9437157400156617,
 'support': 10216}
weighted
{'f1-score': 0.9428609357861484,
 'precision': 0.9450412921434529,
 'recall': 0.9437157400156617,
 'support': 10216}
### report_full
macro
{'f1-score': 0.8181425337362648,
 'precision': 0.9080358767273333,
 'recall': 0.7712280717779727,
 'support': 11704}
micro
{'f1-score': 0.8796532846715328,
 'precision': 0.9437157400156617,
 'recall': 0.8237354750512645,
 'support': 11704}
weighted
{'f1-score': 0.8714276078498072,
 'precision': 0.9427611938119291,
 'recall': 0.8237354750512645,
 'support': 11704}
## test
PPCR: 0.881878
### report
macro
{'f1-score': 0.8688821584511759,
 'precision': 0.8679184942175039,
 'recall': 0.8756891884889126,
 'support': 2292}
micro
{'f1-score': 0.9293193717277487,
 'precision': 0.9293193717277487,
 'recall': 0.9293193717277487,
 'support': 2292}
weighted
{'f1-score': 0.9279276606239152,
 'precision': 0.9286541592741692,
 'recall': 0.9293193717277487,
 'support': 2292}
### report_full
macro
{'f1-score': 0.8077597923894493,
 'precision': 0.8679184942175039,
 'recall': 0.7727980039119674,
 'support': 2599}
micro
{'f1-score': 0.8709875281128602,
 'precision': 0.9293193717277487,
 'recall': 0.8195459792227779,
 'support': 2599}
weighted
{'f1-score': 0.8626973882249023,
 'precision': 0.9239363242916627,
 'recall': 0.8195459792227779,
 'support': 2599}
```

## javascript
### Summary
10 rules, avg.len. 5.3

| | |
|-|-|
|Min support|93|
|Max support|1484|
|Min confidence|0.9343891143798828|
|Max confidence|0.9990653991699219|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 535.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 1430.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 124.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 313.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.934. Support: 221.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 370.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 1484.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 447.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved = ><br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 157.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.984. Support: 93.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.3, "max_conf": 0.9990653991699219, "max_support": 1484, "min_conf": 0.9343891143798828, "min_support": 93, "num_rules": 10}}
```
</details>
