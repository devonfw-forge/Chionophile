using System;
using Devon4Net.WebAPI.Implementation.Domain.Entities;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata;

namespace Devon4Net.WebAPI.Implementation.Domain.Database
{
    public partial class jtqdbContext : DbContext
    {
        public jtqdbContext()
        {
        }

        public jtqdbContext(DbContextOptions<jtqdbContext> options)
            : base(options)
        {
        }

        public virtual DbSet<Accesscode> Accesscode { get; set; }
        public virtual DbSet<Dailyqueue> Dailyqueue { get; set; }
        public virtual DbSet<Visitor> Visitor { get; set; }

        protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
        {
            if (!optionsBuilder.IsConfigured)
            {
                optionsBuilder.UseNpgsql("Host=localhost;Database=jtq_db;Username=jtq_user;Password=admin");
            }
        }

        protected override void OnModelCreating(ModelBuilder modelBuilder)
        {
            modelBuilder.Entity<Accesscode>(entity =>
            {
                entity.ToTable("accesscode");

                entity.Property(e => e.Id).HasColumnName("id");

                entity.Property(e => e.Creationtime).HasColumnName("creationtime");

                entity.Property(e => e.Endtime).HasColumnName("endtime");

                entity.Property(e => e.Idqueue).HasColumnName("idqueue");

                entity.Property(e => e.Idvisitor).HasColumnName("idvisitor");

                entity.Property(e => e.Modificationcounter).HasColumnName("modificationcounter");

                entity.Property(e => e.Starttime).HasColumnName("starttime");

                entity.HasOne(d => d.IdqueueNavigation)
                    .WithMany(p => p.Accesscode)
                    .HasForeignKey(d => d.Idqueue)
                    .OnDelete(DeleteBehavior.ClientSetNull)
                    .HasConstraintName("fk_accesscode_idqueue");

                entity.HasOne(d => d.IdvisitorNavigation)
                    .WithMany(p => p.Accesscode)
                    .HasForeignKey(d => d.Idvisitor)
                    .OnDelete(DeleteBehavior.ClientSetNull)
                    .HasConstraintName("fk_accesscode_idvisitor");
            });

            modelBuilder.Entity<Dailyqueue>(entity =>
            {
                entity.ToTable("dailyqueue");

                entity.Property(e => e.Id).HasColumnName("id");

                entity.Property(e => e.Active)
                    .IsRequired()
                    .HasColumnName("active")
                    .HasDefaultValueSql("true");

                entity.Property(e => e.Attentiontime).HasColumnName("attentiontime");

                entity.Property(e => e.Currentnumber)
                    .HasColumnName("currentnumber")
                    .HasMaxLength(255);

                entity.Property(e => e.Logo)
                    .HasColumnName("logo")
                    .HasMaxLength(255);

                entity.Property(e => e.Minattentiontime)
                    .HasColumnName("minattentiontime")
                    .HasDefaultValueSql("'2025-01-01 00:00:00'::timestamp without time zone");

                entity.Property(e => e.Modificationcounter).HasColumnName("modificationcounter");

                entity.Property(e => e.Name)
                    .HasColumnName("name")
                    .HasMaxLength(255);
            });

            modelBuilder.Entity<Visitor>(entity =>
            {
                entity.ToTable("visitor");

                entity.Property(e => e.Id).HasColumnName("id");

                entity.Property(e => e.Acceptedcommercial)
                    .HasColumnName("acceptedcommercial")
                    .HasDefaultValueSql("false");

                entity.Property(e => e.Acceptedterms).HasColumnName("acceptedterms");

                entity.Property(e => e.Modificationcounter).HasColumnName("modificationcounter");

                entity.Property(e => e.Name)
                    .HasColumnName("name")
                    .HasMaxLength(255);

                entity.Property(e => e.Password)
                    .HasColumnName("password")
                    .HasMaxLength(255);

                entity.Property(e => e.Phonenumber)
                    .HasColumnName("phonenumber")
                    .HasMaxLength(255);

                entity.Property(e => e.Username)
                    .HasColumnName("username")
                    .HasMaxLength(255);

                entity.Property(e => e.Usertype)
                    .HasColumnName("usertype")
                    .HasDefaultValueSql("false");
            });

            OnModelCreatingPartial(modelBuilder);
        }

        partial void OnModelCreatingPartial(ModelBuilder modelBuilder);
    }
}
