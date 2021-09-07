# Model report for file:///tmp/top-repos-quality-repos-ndxw65pu/react-auth-server-admin.git HEAD 703a07fe863fb0eadad6137990df5bf34b5779cb

### Dump

```json
{'created_at': '2021-08-31 23:51:40',
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
 'size': '16.4 kB',
 'tags': [],
 'uuid': 'a17db238-46ff-4c8b-bf6a-2b15a98c32f5',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ndxw65pu/react-auth-server-admin.git 703a07fe863fb0eadad6137990df5bf34b5779cb

# javascript
21 rules, avg.len. 4.3
## train
PPCR: 0.656311
### report
macro
{'f1-score': 0.1996040351471983,
 'precision': 0.20249488203458332,
 'recall': 0.20111410089149814,
 'support': 3021}
micro
{'f1-score': 0.8232373386295928,
 'precision': 0.8232373386295928,
 'recall': 0.8232373386295928,
 'support': 3021}
weighted
{'f1-score': 0.7787938918444227,
 'precision': 0.74879591584269,
 'recall': 0.8232373386295928,
 'support': 3021}
### report_full
macro
{'f1-score': 0.16794240389112522,
 'precision': 0.20249488203458332,
 'recall': 0.1543830900507226,
 'support': 4603}
micro
{'f1-score': 0.6524134312696748,
 'precision': 0.8232373386295928,
 'recall': 0.5402998044753422,
 'support': 4603}
weighted
{'f1-score': 0.5603873891751184,
 'precision': 0.6227666580925373,
 'recall': 0.5402998044753422,
 'support': 4603}
## test
PPCR: 0.657729
### report
macro
{'f1-score': 0.17595108695652173,
 'precision': 0.1766082368958475,
 'recall': 0.18138896682331485,
 'support': 417}
micro
{'f1-score': 0.7553956834532374,
 'precision': 0.7553956834532374,
 'recall': 0.7553956834532374,
 'support': 417}
weighted
{'f1-score': 0.6998357835470754,
 'precision': 0.6658596880154147,
 'recall': 0.7553956834532374,
 'support': 417}
### report_full
macro
{'f1-score': 0.14653391721060893,
 'precision': 0.1766082368958475,
 'recall': 0.13206392919871085,
 'support': 634}
micro
{'f1-score': 0.5994291151284491,
 'precision': 0.7553956834532374,
 'recall': 0.4968454258675079,
 'support': 634}
weighted
{'f1-score': 0.5302124170272459,
 'precision': 0.5969147520541954,
 'recall': 0.4968454258675079,
 'support': 634}
```

## javascript
### Summary
16 rules, avg.len. 4.2

| | |
|-|-|
|Min support|158|
|Max support|751|
|Min confidence|0.9380825757980347|
|Max confidence|0.9977272748947144|

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
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.997. Support: 195.` |
| 2 | `  +1.length ≥ 2<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.997. Support: 189.` |
| 3 | `  +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 194.` |
| 4 | `  +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {IDENTIFIER} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 191.` |
| 5 | `  +1.length ≥ 2<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 220.` |
| 6 | `  +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 204.` |
| 7 | `  •••start_line ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE} and not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 751.` |
| 8 | `  •••start_line ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {INCOMPLETE, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 383.` |
| 9 | `  •••start_line ≥ 10<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 507.` |
| 10 | `  •••start_line ≥ 10<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {QUALIFIED} and not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 158.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = .<br>	∧ +1.length ≥ 2<br>⇒ y = ∅<br>Confidence: 0.998. Support: 205.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {.}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.997. Support: 186.` |
| 13 | `  +1.length ≥ 2<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 190.` |
| 14 | `  +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 167.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 214.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 170.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.1875, "max_conf": 0.9977272748947144, "max_support": 751, "min_conf": 0.9380825757980347, "min_support": 158, "num_rules": 16}}
```
</details>
