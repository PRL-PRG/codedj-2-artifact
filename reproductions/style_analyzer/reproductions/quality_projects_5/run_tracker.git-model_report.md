# Model report for file:///tmp/top-repos-quality-repos-0os27k1t/run_tracker.git HEAD dfe4baa73200ff5f2068520f0e7d04879c50ab15

### Dump

```json
{'created_at': '2021-09-02 09:55:07',
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
 'size': '18.9 kB',
 'tags': [],
 'uuid': 'e5dd84a4-2d84-493a-b482-d652cd5f6f5e',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-0os27k1t/run_tracker.git dfe4baa73200ff5f2068520f0e7d04879c50ab15

# javascript
20 rules, avg.len. 6.4
## train
PPCR: 0.825075
### report
macro
{'f1-score': 0.46115782890847984,
 'precision': 0.48093500277648726,
 'recall': 0.44640221627881804,
 'support': 13155}
micro
{'f1-score': 0.9480805777271,
 'precision': 0.9480805777271,
 'recall': 0.9480805777271,
 'support': 13155}
weighted
{'f1-score': 0.94252969479229,
 'precision': 0.939289374573132,
 'recall': 0.9480805777271,
 'support': 13155}
### report_full
macro
{'f1-score': 0.4163786665529579,
 'precision': 0.48093500277648726,
 'recall': 0.3833572117262079,
 'support': 15944}
micro
{'f1-score': 0.8572115880270801,
 'precision': 0.9480805777271,
 'recall': 0.782237832413447,
 'support': 15944}
weighted
{'f1-score': 0.8211652811726572,
 'precision': 0.8815187743704542,
 'recall': 0.782237832413447,
 'support': 15944}
## test
PPCR: 0.844326
### report
macro
{'f1-score': 0.43390746658864837,
 'precision': 0.46526297237805575,
 'recall': 0.41464464122909905,
 'support': 4122}
micro
{'f1-score': 0.9344978165938864,
 'precision': 0.9344978165938864,
 'recall': 0.9344978165938864,
 'support': 4122}
weighted
{'f1-score': 0.9293551193916161,
 'precision': 0.9274700994102734,
 'recall': 0.9344978165938864,
 'support': 4122}
### report_full
macro
{'f1-score': 0.38434751617117036,
 'precision': 0.46526297237805575,
 'recall': 0.35447213376138004,
 'support': 4882}
micro
{'f1-score': 0.8556197245668592,
 'precision': 0.9344978165938864,
 'recall': 0.7890208930766079,
 'support': 4882}
weighted
{'f1-score': 0.8181074772284159,
 'precision': 0.871651292856313,
 'recall': 0.7890208930766079,
 'support': 4882}
```

## javascript
### Summary
17 rules, avg.len. 6.4

| | |
|-|-|
|Min support|109|
|Max support|2919|
|Min confidence|0.9265159368515015|
|Max confidence|0.9989919066429138|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = "<br>Confidence: 0.971. Support: 222.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.977. Support: 2790.` |
| 3 | `  -1.reserved = .<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 496.` |
| 4 | `  -1.reserved not in {.}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = "<br>Confidence: 0.988. Support: 297.` |
| 5 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {.}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = '<br>Confidence: 0.987. Support: 264.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {., ;}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = ObjectExpression<br>⇒ y = ⏎<br>Confidence: 0.967. Support: 230.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 124.` |
| 8 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ., ;, {}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, ObjectExpression}<br>⇒ y = "<br>Confidence: 0.951. Support: 112.` |
| 9 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ., ;, {}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.length ≤ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, ObjectExpression}<br>	∧ ^1.roles in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 121.` |
| 10 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ., ;, {}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -4.length ≤ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, ObjectExpression}<br>	∧ ^1.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 1447.` |
| 11 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = '<br>Confidence: 0.992. Support: 301.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = {<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 128.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {{}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 166.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {{}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 170.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 109.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 126.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression, JSXElement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.927. Support: 2919.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.352941176470588, "max_conf": 0.9989919066429138, "max_support": 2919, "min_conf": 0.9265159368515015, "min_support": 109, "num_rules": 17}}
```
</details>
