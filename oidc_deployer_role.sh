#!/bin/bash

AWS_REGION="eu-central-1"

AWS_ACCOUNT_ID="$(aws sts get-caller-identity --query Account --output text)"

# Create role for automation
aws iam create-role --role-name githubOIDCCDKDeployerRole  --assume-role-policy-document '{"Version":"2012-10-17","Statement":[{"Sid":"","Effect":"Allow","Principal":{"Federated":"arn:aws:iam::'${AWS_ACCOUNT_ID}':oidc-provider/token.actions.githubusercontent.com"},"Action":"sts:AssumeRoleWithWebIdentity","Condition":{"StringEquals":{"token.actions.githubusercontent.com:sub":"repo:timello/shortener:ref:refs/heads/develop","token.actions.githubusercontent.com:aud":"sts.amazonaws.com"}}}]}'
aws iam attach-role-policy --policy-arn arn:aws:iam::aws:policy/AdministratorAccess --role-name githubOIDCCDKDeployerRole
