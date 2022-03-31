# Generated by Django 4.0.3 on 2022-03-30 15:00

from django.db import migrations, models
import django.db.models.deletion


class Migration(migrations.Migration):

    initial = True

    dependencies = [
    ]

    operations = [
        migrations.CreateModel(
            name='Queue',
            fields=[
                ('id', models.IntegerField(primary_key=True, serialize=False)),
                ('modificationcounter', models.IntegerField()),
                ('name', models.TextField()),
                ('logo', models.TextField()),
                ('currentnumber', models.TextField()),
                ('attentiontime', models.DateField()),
                ('minattentiontime', models.DateField()),
                ('active', models.BooleanField()),
            ],
        ),
        migrations.CreateModel(
            name='Visitor',
            fields=[
                ('id', models.IntegerField(primary_key=True, serialize=False)),
                ('modificationcounter', models.IntegerField()),
                ('username', models.TextField()),
                ('name', models.TextField()),
                ('password', models.TextField()),
                ('phonenumber', models.TextField()),
                ('acceptedcommecial', models.BooleanField()),
                ('acceptedterms', models.BooleanField()),
                ('usertype', models.BooleanField()),
            ],
        ),
        migrations.CreateModel(
            name='AccessCode',
            fields=[
                ('id', models.IntegerField(primary_key=True, serialize=False)),
                ('modificationcounter', models.IntegerField()),
                ('ticketnumber', models.TextField()),
                ('creationtime', models.DateField()),
                ('starttime', models.DateField()),
                ('endtime', models.DateField()),
                ('queue', models.ForeignKey(null=True, on_delete=django.db.models.deletion.SET_NULL, to='jtq_rest.queue')),
                ('visitor', models.ForeignKey(null=True, on_delete=django.db.models.deletion.SET_NULL, to='jtq_rest.visitor')),
            ],
        ),
    ]
